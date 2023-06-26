use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::app_config::{AppConfig};
use crate::layout_config::{LayoutConfig, load_layouts};
use crate::overlay::show_overlay;

use gio::SimpleAction;
use glib::{Sender, MainContext, Priority, clone};
use gtk::{prelude::*};
use gtk::{Window, WindowType, traits::WidgetExt};

enum OverlaySignals {
    ChangeActiveState(bool, LayoutConfig),
    Save(LayoutConfig),
    Remove(LayoutConfig)
}

pub struct App {
    config: Arc<AppConfig>,
}

impl App {
    pub fn new(app_config: AppConfig) -> Self {
        Self { 
            config: Arc::new(app_config), 
        }
    }

    pub fn show(&mut self) {
        let (sender, receiver) = MainContext::channel(Priority::default());
        let mut win_senders_map = HashMap::<String, Sender<bool>>::new();
        receiver.attach(None, move |signal| {
            match signal {
                OverlaySignals::ChangeActiveState(new_state, overlay_config) => {
                    if new_state {
                        println!("Activate the overlay: {}", overlay_config.name());
                        let (win_sender, win_receiver) = MainContext::channel(Priority::default());
                        let cloned_config = overlay_config.clone();

                        win_senders_map.insert(overlay_config.name(), win_sender.clone());
                        std::thread::spawn(move || {
                            glib::MainContext::default().invoke(move || {
                                show_overlay(&cloned_config.clone(), win_receiver);
                            });
                            0
                        });
                    } else {
                        println!("Disable the overlay: {}", overlay_config.name());
                        if let Some(sender) = win_senders_map.get(&overlay_config.name()) {
                            sender.send(true).unwrap();
                        }
                    }
                },
                _ => {}
            }

            glib::Continue(true)
        });

        let config = LayoutConfig::from_file("./config.yaml")
            .expect("Could not parse the configuration");

        let window = Window::new(WindowType::Toplevel);
        window.set_size_request(1000, 700);

        let layout = gtk::Paned::new(gtk::Orientation::Horizontal);
        let sidebar = self.create_sidebar();
        let app_content_frame = gtk::Frame::new(None);

        layout.pack1(&sidebar, false, false);
        layout.pack2(&app_content_frame, true, false);

        let overlay_widget = OverlayInfos::new(
            config,
            sender
        );
        app_content_frame.add(&overlay_widget.ui());

        window.add(&layout);

        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    fn create_treeview_entries(&self) -> gtk::TreeStore {
        // Creation of a model with two rows.
        let model = gtk::TreeStore::new(&[String::static_type()]);

        let configs = load_layouts(&self.config);

        for entry in configs.iter() {
            let _ = model.insert_with_values(None, None, &[(0, &entry.name())]); // The iterator returned will be used to handle folders
        }
        model
    }

    fn append_treeview_column(&self, tree: &gtk::TreeView, id: i32) {
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();

        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&column, &cell, "text", id);

        tree.append_column(&column);
    }

    fn create_sidebar(&self) -> gtk::Frame {
        let sidebar_frame = gtk::Frame::new(None);
        sidebar_frame.set_size_request(200, 700);

        let treeview = gtk::TreeView::new();
        treeview.set_headers_visible(false);
        self.append_treeview_column(&treeview, 0);

        let model = self.create_treeview_entries();

        treeview.set_model(Some(&model));

        sidebar_frame.add(&treeview);

        sidebar_frame
    }
}

struct OverlayInfos {
    current_overlay: LayoutConfig,
    sender: Sender<OverlaySignals>,
    active_state_switch: gtk::Switch
}

impl OverlayInfos {
    pub fn new(layout_config: LayoutConfig, sender: Sender<OverlaySignals>) -> Self {
        let switch = gtk::Switch::builder()
            .build();
        
        Self {
            current_overlay: layout_config,
            active_state_switch: switch,
            sender
        }
    }

    fn ui(&self) -> gtk::Widget{
        let infos_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin(15)
            .build();
    
        let header = self.create_header();     
    
        infos_container.add(&header);
        infos_container.into()
    }
    
    fn create_header<'a>(&self) -> gtk::Widget {
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
    
        let title_text = gtk::Label::builder()
            .label(&self.current_overlay.name())
            .build();
        title_text
            .style_context()
            .add_class("overlay-title");
    
        
        let cloned_config = self.current_overlay.clone();
        let cloned_sender = self.sender.clone();
        self.active_state_switch.connect_state_set(move |_, new_state| {
            let signal = OverlaySignals::ChangeActiveState(new_state, cloned_config.clone());
            cloned_sender.send(signal).unwrap();
            Inhibit(true)
        });
    
        header.add(&title_text);
        header.pack_end(&self.active_state_switch, false, false, 0);

        header.into()
    }
}



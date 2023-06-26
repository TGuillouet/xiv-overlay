use std::collections::HashMap;
use std::sync::Arc;

use crate::app_config::{AppConfig};
use crate::layout_config::{LayoutConfig, load_layouts};
use crate::overlay::show_overlay;
use crate::ui::OverlaySignals;
use crate::ui::overlay_infos::OverlayInfos;
use crate::ui::sidebar::Sidebar;

use glib::{Sender, MainContext, Priority};
use gtk::{prelude::*};
use gtk::{Window, WindowType, traits::WidgetExt};

pub struct App {
    config: Arc<AppConfig>,
    sender: Option<Sender<OverlaySignals>>
}

impl App {
    pub fn new(app_config: AppConfig) -> Self {
        Self { 
            config: Arc::new(app_config),
            sender: None
        }
    }

    pub fn init(&mut self) {
        let (sender, receiver) = MainContext::channel(Priority::default());
        self.sender = Some(sender.clone());

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
    }

    pub fn show(&self) {
        let window = Window::new(WindowType::Toplevel);
        window.set_size_request(1000, 700);

        let layout = gtk::Paned::new(gtk::Orientation::Horizontal);
        let mut sidebar = Sidebar::new();
        let app_content_frame = gtk::Frame::new(None);

        layout.pack1(&sidebar.ui(&self.config), false, false);
        layout.pack2(&app_content_frame, true, false);

        let overlay_widget = OverlayInfos::new(
            sidebar.get_layout_at(0),
            self.sender.clone().unwrap()
        );
        app_content_frame.add(&overlay_widget.ui());

        window.add(&layout);

        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }
}

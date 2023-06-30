use async_channel::Sender;
use gtk::prelude::*;
use gtk::{traits::{TreeViewExt, WidgetExt}, prelude::TreeStoreExtManual};

use crate::app::AppAction;
use crate::layout_config::get_layout_by_name;
use crate::layout_config::LayoutConfig;

pub struct Sidebar {
    pub frame: gtk::Frame,
    treeview: gtk::TreeView,
    
    add_menu_item: gtk::MenuItem,
    remove_menu_item: gtk::MenuItem,
}

impl Sidebar {
    pub fn new(event_sender: Sender<AppAction>) -> Self {
        let sidebar_frame = gtk::Frame::new(None);
        sidebar_frame.set_size_request(200, 700);
    
        let treeview = gtk::TreeView::new();
        treeview.set_headers_visible(false);
        treeview.set_activate_on_single_click(true);
        Sidebar::append_treeview_column(&treeview, 0);
        sidebar_frame.add(&treeview);

        let item_actions_menu = gtk::Menu::new();
        let remove_menu_item = gtk::MenuItem::with_label("Delete");
        item_actions_menu.append(&remove_menu_item);
        remove_menu_item.show();

        let treeview_actions_menu = gtk::Menu::new();
        let add_menu_item = gtk::MenuItem::with_label("Add new config");
        treeview_actions_menu.append(&add_menu_item);
        add_menu_item.show();

        let sidebar = Self {
            frame: sidebar_frame,
            treeview,

            add_menu_item,
            remove_menu_item
        };

        sidebar.setup_signals(treeview_actions_menu, item_actions_menu, event_sender);

        sidebar
    }

    fn setup_signals(&self, treeview_menu: gtk::Menu, treeview_item_menu: gtk::Menu, event_sender: Sender<AppAction>) {
        let event_sender_clone = event_sender.clone();
        self.add_menu_item.connect_activate(move |_item| {
            let _ = glib::MainContext::default().block_on(event_sender_clone.send(AppAction::NewOverlay));
        });
        
        let event_sender_clone = event_sender.clone();
        let treeview = self.treeview.clone();
        self.remove_menu_item.connect_activate(move |_item| {
            let (path, _) = treeview.cursor();
            let treeview_model = treeview.model().unwrap();
            let iter = treeview_model.iter(&path.unwrap()).unwrap();
            let value = treeview_model.value(&iter, 0).get::<String>().unwrap();

            if let Ok(overlay) = get_layout_by_name(&value) {
                let _ = glib::MainContext::default().block_on(event_sender_clone.send(AppAction::DeleteOverlay(overlay)));
                let _ = glib::MainContext::default().block_on(event_sender_clone.send(AppAction::LoadOverlaysList));
            }
        });

        let event_sender_clone = event_sender.clone();
        self.treeview.connect_row_activated(move |view, path, _column| {
            let model = view.model().unwrap();
            let iter = model.iter(path).unwrap();
            let value = model.value(&iter, 0).get::<String>().unwrap();
    
            if let Ok(overlay) = get_layout_by_name(&value) {
                let _ = glib::MainContext::default().block_on(event_sender_clone.send(AppAction::SelectOverlay(overlay)));
            }
        });

        self.treeview.connect_button_press_event(move |treeview, event| {
            if event.button() == 3 {
                let selected_item = treeview.path_at_pos(event.position().0 as i32, event.position().1 as i32)
                    .map(|(path, _, _, _)| {
                        treeview.set_cursor(&path.unwrap(), None::<&gtk::TreeViewColumn>, false);
                        treeview.grab_focus();
                        treeview_item_menu.popup_at_pointer(Some(&event));
                    });

                // If we do not find any item, show the other menu
                if selected_item.is_none() {
                    treeview_menu.popup_at_pointer(Some(&event));
                }

                return Inhibit(true);
            }

            Inhibit(false)
        });
    }

    pub fn display_overlays_list(&self, overlays_list: Vec<LayoutConfig>) {
        let model = Sidebar::create_treeview_entries(overlays_list);
        self.treeview.set_model(Some(&model));
    }

    fn create_treeview_entries(overlays_list: Vec<LayoutConfig>) -> gtk::TreeStore {
        // Creation of a model with two rows.
        let model = gtk::TreeStore::new(&[String::static_type()]);

        for entry in overlays_list.iter() {
            let _ = model.insert_with_values(None, None, &[(0, &entry.name())]); // The iterator returned will be used to handle folders
        }
        model
    }

    fn append_treeview_column(tree: &gtk::TreeView, id: i32) {
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();

        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&column, &cell, "text", id);

        tree.append_column(&column);
    }
}
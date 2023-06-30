use async_channel::Sender;
use gtk::prelude::*;
use gtk::{traits::{TreeViewExt, WidgetExt}, prelude::TreeStoreExtManual};

use crate::app::AppAction;
use crate::layout_config::get_layout_by_name;
use crate::layout_config::LayoutConfig;

pub struct Sidebar {
    pub frame: gtk::Frame,
    treeview: gtk::TreeView
}

impl Sidebar {
    pub fn new(event_sender: Sender<AppAction>) -> Self {
        let sidebar_frame = gtk::Frame::new(None);
        sidebar_frame.set_size_request(200, 700);
    
        let treeview = gtk::TreeView::new();
        treeview.set_headers_visible(false);
        treeview.set_activate_on_single_click(true);
        Sidebar::append_treeview_column(&treeview, 0);
    
        let cloned_sender = event_sender.clone();
        treeview.connect_row_activated(move |view, path, _column| {
            let model = view.model().unwrap();
            let iter = model.iter(path).unwrap();
            let value = model.value(&iter, 0).get::<String>().unwrap();
    
            if let Ok(overlay) = get_layout_by_name(&value) {
                let _ = glib::MainContext::default().block_on(cloned_sender.send(AppAction::SelectOverlay(overlay)));
            }
        });
    
        sidebar_frame.add(&treeview);
    
        Self {
            frame: sidebar_frame,
            treeview
        }
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
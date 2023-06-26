use gtk::prelude::*;
use gtk::{traits::{TreeViewExt, WidgetExt}, prelude::TreeStoreExtManual};

use crate::{layout_config::{LayoutConfig, load_layouts}, app_config::AppConfig};

pub struct Sidebar {
    layout_list: Vec<LayoutConfig>
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            layout_list: Vec::new()
        }
    }

    pub fn ui(&mut self, app_config: &AppConfig) -> gtk::Frame {
        let sidebar_frame = gtk::Frame::new(None);
        sidebar_frame.set_size_request(200, 700);

        let treeview = gtk::TreeView::new();
        treeview.set_headers_visible(false);
        self.append_treeview_column(&treeview, 0);

        let model = self.create_treeview_entries(&app_config);

        treeview.set_model(Some(&model));

        sidebar_frame.add(&treeview);

        sidebar_frame
    }

    fn create_treeview_entries(&mut self, app_config: &AppConfig) -> gtk::TreeStore {
        // Creation of a model with two rows.
        let model = gtk::TreeStore::new(&[String::static_type()]);

        self.layout_list = load_layouts(&app_config);

        for entry in self.layout_list.iter() {
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

    pub fn get_layout_at(&self, index: usize) -> LayoutConfig {
        self.layout_list.clone().into_iter().nth(index).unwrap()
    }
}
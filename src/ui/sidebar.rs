use glib::Sender;
use gtk::prelude::*;
use gtk::{traits::{TreeViewExt, WidgetExt}, prelude::TreeStoreExtManual};

use crate::layout_config::get_layout_by_name;
use crate::{layout_config::{LayoutConfig, load_layouts}};

pub struct Sidebar {
    layout_list: Vec<LayoutConfig>,
    change_selection_sender: Sender<LayoutConfig>
}

impl Sidebar {
    pub fn new(change_selection_sender: Sender<LayoutConfig>) -> Self {
        Self {
            layout_list: Vec::new(),
            change_selection_sender
        }
    }

    pub fn ui(&mut self) -> gtk::Frame {
        let sidebar_frame = gtk::Frame::new(None);
        sidebar_frame.set_size_request(200, 700);

        let treeview = gtk::TreeView::new();
        treeview.set_headers_visible(false);
        treeview.set_activate_on_single_click(true);
        self.append_treeview_column(&treeview, 0);

        let model = self.create_treeview_entries();

        treeview.set_model(Some(&model));
        let cloned_sender = self.change_selection_sender.clone();
        treeview.connect_row_activated(move |view, path, _column| {
            let model = view.model().unwrap();
            let iter = model.iter(path).unwrap();
            let value = model.value(&iter, 0).get::<String>().unwrap();

            if let Ok(overlay) = get_layout_by_name(&value) {
                let _ = cloned_sender.send(overlay);
            }
        });

        sidebar_frame.add(&treeview);

        sidebar_frame
    }

    fn create_treeview_entries(&mut self) -> gtk::TreeStore {
        // Creation of a model with two rows.
        let model = gtk::TreeStore::new(&[String::static_type()]);

        self.layout_list = load_layouts();

        for entry in self.layout_list.iter() {
            let _ = model.insert_with_values(None, None, &[(0, &entry.name())]); // The iterator returned will be used to handle folders
        }
        model
    }

    fn append_treeview_column(&self, tree: &gtk::TreeView, id: i32) {
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();

        // gtk::prelude::TreeViewColumnExt::set_cell_data_func(&column, &cell, Some(Box::new(Sidebar::set_cell_data)));
        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&column, &cell, "text", id);

        tree.append_column(&column);
    }

    // fn set_cell_data(column: &gtk::TreeViewColumn, renderer: &gtk::CellRenderer, model: &gtk::TreeModel, iter: &gtk::TreeIter) {
        
    // }

    pub fn get_layout_at(&self, index: usize) -> LayoutConfig {
        self.layout_list.clone().into_iter().nth(index).unwrap()
    }
}
use crate::layout_config::LayoutConfig;

use gtk::prelude::*;
use gtk::{Window, WindowType, traits::WidgetExt};

fn create_and_fill_model() -> gtk::TreeStore {
    // Creation of a model with two rows.
    let model = gtk::TreeStore::new(&[u32::static_type(), String::static_type()]);

    // Filling up the tree view.
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    for (i, entry) in entries.iter().enumerate() {
        let iter = model.insert_with_values(None, None, &[(1, &entry)]);

        for item  in &["Test 1", "Test 2"] {
            model.insert_with_values(Some(&iter), None, &[(1, &item)]);
        }
    }
    model
}

fn append_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
    gtk::prelude::TreeViewColumnExt::add_attribute(&column, &cell, "text", id);

    tree.append_column(&column);
}

fn create_sidebar(config: &LayoutConfig) -> gtk::Frame {
    let sidebar_frame = gtk::Frame::new(None);
    sidebar_frame.set_size_request(200, 700);

    // TODO: Load the currently created layouts

    let treeview = gtk::TreeView::new();
    treeview.set_headers_visible(false);
    append_column(&treeview, 1);

    let model = create_and_fill_model();

    treeview.set_model(Some(&model));

    sidebar_frame.add(&treeview);

    sidebar_frame
}

pub fn show_app(config: &LayoutConfig) {
    let window = Window::new(WindowType::Toplevel);
    window.set_size_request(1000, 700);

    let layout = gtk::Paned::new(gtk::Orientation::Horizontal);
    let sidebar = create_sidebar(config);
    let app_content_frame = gtk::Frame::new(None);

    layout.pack1(&sidebar, false, false);
    layout.pack2(&app_content_frame, true, false);

    window.add(&layout);

    window.show_all();
}
use crate::app_config::AppConfig;
use crate::layout_config::LayoutConfig;

use gtk::prelude::*;
use gtk::{Window, WindowType, traits::WidgetExt};

fn load_layouts(config: &AppConfig) -> Vec<LayoutConfig> {
    let layout_config_path = config.layouts_config_path();
    let mut layout_configs: Vec<LayoutConfig> = Vec::new();
    let read_result = std::fs::read_dir(layout_config_path).expect("Could not read the layouts path");
    for file_result in read_result {
        match file_result {
            Ok(file) => {
                if let Ok(config) = LayoutConfig::from_file(file.path().to_str().unwrap().to_string()) {
                    layout_configs.push(
                        config
                    );
                }
            },
            Err(_) => {},
        }
    }
    layout_configs
}

fn create_and_fill_model(config: &AppConfig) -> gtk::TreeStore {
    // Creation of a model with two rows.
    let model = gtk::TreeStore::new(&[String::static_type()]);

    let configs = load_layouts(config);

    for entry in configs.iter() {
        let iter = model.insert_with_values(None, None, &[(0, &entry.name())]);
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

fn create_sidebar(config: &AppConfig) -> gtk::Frame {
    let sidebar_frame = gtk::Frame::new(None);
    sidebar_frame.set_size_request(200, 700);

    let treeview = gtk::TreeView::new();
    treeview.set_headers_visible(false);
    append_column(&treeview, 0);

    let model = create_and_fill_model(config);

    treeview.set_model(Some(&model));

    sidebar_frame.add(&treeview);

    sidebar_frame
}

pub fn show_app(config: &AppConfig) {
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
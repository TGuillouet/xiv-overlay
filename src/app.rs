use crate::app_config::AppConfig;
use crate::layout_config::LayoutConfig;

use gdk::Screen;
use gtk::{prelude::*, StyleContext};
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
        let _ = model.insert_with_values(None, None, &[(0, &entry.name())]); // The iterator returned will be used to handle folders
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

fn display_overlay_infos(overlay_infos: &LayoutConfig) -> gtk::Widget{
    let infos_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin(15)
        .build();

    let header = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .build();

    let title_text = gtk::Label::builder()
        .label(&overlay_infos.name())
        .build();
    title_text
        .style_context()
        .add_class("overlay-title");

    let state_switch = gtk::Switch::builder()
        .build();

    header.add(&title_text);
    header.pack_end(&state_switch, false, false, 0);

    infos_container.add(&header);
    infos_container.into()
}

pub fn show_app(config: &AppConfig) {
    let window = Window::new(WindowType::Toplevel);
    window.set_size_request(1000, 700);

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path("./styles/app.css")
        .expect("Could not load the stylesheet");
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not fetch the gdk screen"), 
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );

    let layout = gtk::Paned::new(gtk::Orientation::Horizontal);
    let sidebar = create_sidebar(config);
    let app_content_frame = gtk::Frame::new(None);

    layout.pack1(&sidebar, false, false);
    layout.pack2(&app_content_frame, true, false);

    let config = LayoutConfig::from_file("./config.yaml")
        .expect("Could not parse the configuration");
    app_content_frame.add(&display_overlay_infos(&config));

    window.add(&layout);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}
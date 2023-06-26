mod layout_config;
mod overlay;
mod app;
mod app_config;
mod ui;

use app::App;
use gdk::Screen;
use gtk::{traits::CssProviderExt, StyleContext};

use crate::app_config::AppConfig;

fn main() {
    let app_config = AppConfig::default();

    if !app_config.layouts_config_path().exists()  {
        std::fs::create_dir_all(app_config.layouts_config_path())
            .expect("Could not create the layout directory");
    }

    gtk::init().unwrap();

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path("./styles/app.css")
        .expect("Could not load the stylesheet");
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not fetch the gdk screen"), 
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );

    let mut app = App::new(app_config);
    app.init();
    app.show();

    gtk::main();
}

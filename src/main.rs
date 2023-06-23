mod layout_config;
mod overlay;
mod app;
mod app_config;

use crate::{overlay::show_overlay, layout_config::LayoutConfig, app::show_app, app_config::AppConfig};

fn main() {
    let app_config = AppConfig::default();
    let config = LayoutConfig::from_file("./config.yaml")
        .expect("Could not parse the configuration");

    if !app_config.layouts_config_path().exists()  {
        std::fs::create_dir_all(app_config.layouts_config_path())
            .expect("Could not create the layout directory");
    }

    gtk::init().unwrap();

    show_app(&app_config);

    show_overlay(&config);

    gtk::main();
}

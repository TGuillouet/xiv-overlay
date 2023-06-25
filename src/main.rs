mod layout_config;
mod overlay;
mod app;
mod app_config;

use crate::{app::show_app, app_config::AppConfig};

fn main() {
    let app_config = AppConfig::default();

    if !app_config.layouts_config_path().exists()  {
        std::fs::create_dir_all(app_config.layouts_config_path())
            .expect("Could not create the layout directory");
    }

    gtk::init().unwrap();

    show_app(&app_config);

    gtk::main();
}

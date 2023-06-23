mod layout_config;
mod overlay;
mod app;
mod app_config;

use crate::{overlay::show_overlay, layout_config::LayoutConfig, app::show_app};

fn main() {
    let config = LayoutConfig::from_file("./config.yaml")
        .expect("Could not parse the configuration");
    println!("{:?}", config);

    gtk::init().unwrap();

    show_app(&config);

    show_overlay(&config);

    gtk::main();
}

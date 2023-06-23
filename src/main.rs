mod config;
mod overlay;

use crate::{overlay::show_overlay, config::Config};

fn main() {
    let config = Config::from_file("./config.yaml")
        .expect("Could not parse the configuration");
    println!("{:?}", config);

    show_overlay(&config);
}

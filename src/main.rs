extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod layout_config;
mod overlay;
mod app;
mod app_config;
mod ui;
mod errors;

use app::App;
use gdk::Screen;
use gtk::{traits::CssProviderExt, StyleContext};

use crate::app_config::AppConfig;

fn main() {
    pretty_env_logger::init();

    let app_config = AppConfig::default();

    if !app_config.layouts_config_path().exists()  {
        std::fs::create_dir_all(app_config.layouts_config_path())
            .expect("Could not create the layout directory");
    }

    glib::set_program_name("Xiv Overlay".into());
    glib::set_application_name("Xiv Overlay");

    gtk::init().unwrap();

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path("./styles/app.css")
        .expect("Could not load the stylesheet");
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not fetch the gdk screen"), 
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );

    let (sender, receiver) = async_channel::unbounded();

    let mut app = App::new(sender.clone());

    let event_handler = async move {
        while let Ok(event) = receiver.recv().await {
            match event {
                app::AppAction::LoadOverlaysList => app.load_overlays_list(),
                app::AppAction::SelectOverlay(overlay) => app.display_overlay_details(overlay),
                app::AppAction::ToggleOverlay(new_state, overlay) => app.toggle_overlay(new_state, overlay),
                app::AppAction::SaveOverlay(mut overlay) => app.save_overlay(&mut overlay),
                app::AppAction::DeleteOverlay(overlay) => app.delete_overlay(overlay),
                app::AppAction::NewOverlay => app.new_overlay(),
            }
        }
    };

    glib::MainContext::default().spawn_local(event_handler);

    gtk::main();
}

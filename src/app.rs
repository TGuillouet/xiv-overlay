use std::{collections::HashMap};

use async_channel::Sender;
use gtk::{traits::{WidgetExt, ContainerExt, EntryExt, SpinButtonExt}, Inhibit};

use crate::{layout_config::{LayoutConfig, load_layouts, save_overlay, remove_overlay_file}, ui::AppContainer, overlay::show_overlay};

pub enum AppAction {
    NewOverlay,
    LoadOverlaysList,
    SelectOverlay(LayoutConfig),
    ToggleOverlay(bool, LayoutConfig),
    SaveOverlay(LayoutConfig),
    DeleteOverlay(LayoutConfig)
}

pub struct WindowState {
    pub displayed_overlays: HashMap<String, glib::Sender<bool>>,
    pub event_sender: Sender<AppAction>
}

pub struct App {
    window: gtk::Window,
    app_container: AppContainer,
    state: WindowState
}

impl App {
    pub fn new(sender: Sender<AppAction>) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_size_request(1000, 700);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let state = WindowState {
            displayed_overlays: HashMap::default(),
            event_sender: sender.clone()
        };

        let app_container = AppContainer::new(sender.clone());

        window.add(&app_container.container);

        let mut app = Self { 
            window,
            app_container,
            state
        };

        app.show();
        app.display_active_overlays();

        app
    }

    fn show(&self) {
        self.window.show_all();
        
        self.app_container.set_details_visible(false);
        
        let tx = self.state.event_sender.clone();
        glib::MainContext::default().spawn_local(async move {
            let _ = tx.send(AppAction::LoadOverlaysList).await;
        });
    }

    fn display_active_overlays(&mut self) {
        let all_overlays = load_layouts();
        for overlay in all_overlays {
            if !overlay.is_active() {
                continue;
            }

            let (win_sender, win_receiver) = glib::MainContext::channel(glib::Priority::default());
            let overlay_cloned = overlay.clone();

            self.state.displayed_overlays.insert(overlay_cloned.name(), win_sender.clone());
            glib::MainContext::default().invoke(move || {
                show_overlay(&overlay_cloned.clone(), win_receiver);
            });
        }
    }

    pub fn load_overlays_list(&self) {
        println!("Loading the overlays list");
        let overlays = load_layouts();
        self.app_container.sidebar.display_overlays_list(overlays);
    }

    pub fn display_overlay_details(&mut self, overlay: LayoutConfig) {
        println!("Displaying the overlay details of {:?}", overlay);
        self.app_container.set_details_visible(true);
        self.app_container.overlay_details.set_current_overlay(overlay);
    }

    pub fn toggle_overlay(&mut self, new_state: bool, overlay: LayoutConfig) {
        println!("Toggle overlay to {:?} {}", overlay.name(), new_state);

        if new_state {
            let (win_sender, win_receiver) = glib::MainContext::channel(glib::Priority::default());
            let overlay_cloned = overlay.clone();

            self.state.displayed_overlays.insert(overlay_cloned.name(), win_sender.clone());
            glib::MainContext::default().invoke(move || {
                show_overlay(&overlay_cloned.clone(), win_receiver);
            });
        } else {
            if let Some(sender) = self.state.displayed_overlays.get(&overlay.name()) {
                sender.send(true).unwrap();
            }
        }

        let mut new_overlay = overlay.clone();
        new_overlay.set_active(new_state);
        self.save_overlay(&mut new_overlay)
    }

    pub fn save_overlay(&mut self, overlay: &mut LayoutConfig) {
        let overlay_details = &self.app_container.overlay_details;
        let new_name = overlay_details.name_entry.text();
        // Remove the old overlay if the name changed
        if new_name != overlay.name() {
            remove_overlay_file(overlay.get_file_name());
            overlay.set_name(new_name);
        }

        overlay.set_url(overlay_details.url_entry.text());
        overlay.set_x(overlay_details.x_pos_spin.value_as_int());
        overlay.set_y(overlay_details.y_pos_spin.value_as_int());
        overlay.set_width(overlay_details.width_spin.value_as_int());
        overlay.set_height(overlay_details.height_spin.value_as_int());
        
        save_overlay(overlay.clone());

        self.display_overlay_details(overlay.clone())
    }

    pub fn delete_overlay(&self, overlay: LayoutConfig) {
        remove_overlay_file(overlay.get_file_name());
    }

    pub fn new_overlay(&mut self) {
        self.display_overlay_details(LayoutConfig::default());
    }
}

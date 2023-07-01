use std::{collections::HashMap};

use async_channel::Sender;
use gtk::prelude::*;

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

            self.open_overlay(&overlay);
        }
    }

    pub fn load_overlays_list(&self) {
        info!("Loading the overlays list");

        let overlays = load_layouts();
        self.app_container.sidebar.display_overlays_list(overlays);
    }

    pub fn display_overlay_details(&mut self, overlay: LayoutConfig) {
        info!("Displaying the overlay details of {:?}", overlay);

        self.app_container.set_details_visible(true);
        self.app_container.overlay_details.set_current_overlay(overlay);
    }

    pub fn toggle_overlay(&mut self, new_state: bool, overlay: LayoutConfig) {
        info!("Toggle overlay to {:?} {}", overlay.name(), new_state);

        if new_state {
            self.open_overlay(&overlay);
        } else {
            self.close_overlay(&overlay);
        }

        let mut new_overlay = overlay.clone();
        new_overlay.set_active(new_state);
        self.save_overlay(&mut new_overlay)
    }

    pub fn save_overlay(&mut self, overlay: &mut LayoutConfig) {
        let overlay_details = &self.app_container.overlay_details;
        
        let need_reload = 
        overlay_details.clickthrough_check.is_active() != overlay.is_clickthrough()||
        overlay_details.movable_check.is_active() != overlay.is_decoraded();
        if need_reload {
            self.close_overlay(&overlay);
        }
        
        let old_overlay= overlay.clone();
        let need_delete = overlay_details.name_entry.text() != old_overlay.name();
        if need_delete {
            overlay.set_name(overlay_details.name_entry.text());
        }

        overlay.set_url(overlay_details.url_entry.text());
        overlay.set_x(overlay_details.x_pos_spin.value_as_int());
        overlay.set_y(overlay_details.y_pos_spin.value_as_int());
        overlay.set_width(overlay_details.width_spin.value_as_int());
        overlay.set_height(overlay_details.height_spin.value_as_int());
        overlay.set_is_clickthrough(overlay_details.clickthrough_check.is_active());
        overlay.set_is_decorated(overlay_details.movable_check.is_active());
        
        match save_overlay(overlay.clone()) {
            Ok(_) => {
                // Remove the old overlay if the name changed
                if need_delete {
                    self.delete_overlay(&old_overlay);
                }

                if need_reload {
                    self.open_overlay(&overlay);
                }
        
                self.display_overlay_details(overlay.clone());
        
                info!("Overlay {} saved !", overlay.name());
            },
            Err(error) => {
                error!("Could not save the overlay ! Error {:?}", error);
                self.show_dialog("Error", error.to_string().as_str());
            },
        }

    }

    pub fn delete_overlay(&self, overlay: &LayoutConfig) {
        if let Err(error) = remove_overlay_file(overlay.get_file_name()) {
            self.show_dialog("Error", error.to_string().as_str());
        }
    }

    pub fn new_overlay(&mut self) {
        self.display_overlay_details(LayoutConfig::default());
    }

    pub fn close_overlay(&self, overlay: &LayoutConfig) {
        if let Some(sender) = self.state.displayed_overlays.get(&overlay.name()) {
            sender.send(true).unwrap();
        }
    }

    fn open_overlay(&mut self, overlay: &LayoutConfig) {
        let (win_sender, win_receiver) = glib::MainContext::channel(glib::Priority::default());
        let overlay_cloned = overlay.clone();

        self.state.displayed_overlays.insert(overlay_cloned.name(), win_sender.clone());
        glib::MainContext::default().invoke(move || {
            show_overlay(&overlay_cloned.clone(), win_receiver);
        });
    }

    fn show_dialog(&self, title: &str, message: &str) -> () {
        let dialog_window = gtk::MessageDialog::builder()
            .title(title)
            .message_type(gtk::MessageType::Error)
            .text(message)
            .build();
        dialog_window.show();
    }
}

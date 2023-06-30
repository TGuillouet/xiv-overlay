use async_channel::Sender;
use glib::SignalHandlerId;
use gtk::{prelude::*};

use crate::{layout_config::LayoutConfig, app::AppAction};

pub struct OverlayDetails {
    // current_overlay: Option<LayoutConfig>,
    event_sender: Sender<AppAction>,

    pub container: gtk::Box,
    title: gtk::Label,
    active_state_switch: gtk::Switch,
    
    pub name_entry: gtk::Entry,
    save_button: gtk::Button,

    switch_handler_id: Option<SignalHandlerId>,
    save_handler_id: Option<SignalHandlerId>
}

impl OverlayDetails {
    pub fn new(sender: Sender<AppAction>) -> Self {
        let infos_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin(15)
            .build();

        let mut overlay_details = Self {
            // current_overlay: None,
            event_sender: sender.clone(),

            container: infos_container,
            title: gtk::Label::default(),
            active_state_switch: gtk::Switch::default(),

            name_entry: gtk::Entry::default(),
            save_button: gtk::Button::builder().label("Save").build(),

            switch_handler_id: None,
            save_handler_id: None
        };
    
        let header = overlay_details.create_header();
        let form = overlay_details.create_form();
    
        overlay_details.container.add(&header);
        overlay_details.container.add(&form);

        overlay_details
    }
    
    fn create_header(&mut self) -> gtk::Widget {
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();

        self.title
            .style_context()
            .add_class("overlay-title");
        
    
        header.add(&self.title);
        header.pack_end(&self.active_state_switch, false, false, 0);

        header.into()
    }

    fn create_form(&self) -> gtk::Widget {
        let form_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        form_box.add(&self.name_entry);
        form_box.add(&self.save_button);

        form_box.into()
    }

    pub fn set_current_overlay(&mut self, overlay: LayoutConfig) {
        // Important: Disable the events BEFORE assigning new values :)
        if let Some(signal_handler) = self.switch_handler_id.take() {
            self.active_state_switch.disconnect(signal_handler);
        }
        if let Some(signal_handler) = self.save_handler_id.take() {
            self.save_button.disconnect(signal_handler);
        }

        // Update the form entries
        self.title.set_text(&overlay.name());
        self.name_entry.set_text(&overlay.name());
        self.active_state_switch.set_state(overlay.is_active());
        
        let overlay_cloned = overlay.clone();
        let event_sender = self.event_sender.clone();
        self.switch_handler_id = Some(
            self.active_state_switch.connect_state_set(move |_, new_state| {
                let event_sender = event_sender.clone();
                let overlay = overlay_cloned.clone();
                glib::MainContext::default().block_on(async move {
                    let _ = event_sender.send(AppAction::ToggleOverlay(new_state, overlay.clone())).await;
                });
                Inhibit(true)
            })
        );

        let overlay_cloned = overlay.clone();
        let event_sender = self.event_sender.clone();
        self.save_handler_id = Some(
            self.save_button.connect_clicked(move |_| {
                let event_sender = event_sender.clone();
                let overlay_cloned = overlay_cloned.clone();
                glib::MainContext::default().block_on(async move {
                    let _ = event_sender.send(AppAction::SaveOverlay(overlay_cloned)).await;
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let _ = event_sender.send(AppAction::LoadOverlaysList).await;
                })
            })
        );

        // self.current_overlay = Some(overlay)
    }
}



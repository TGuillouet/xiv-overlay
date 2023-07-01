use async_channel::Sender;
use glib::SignalHandlerId;
use gtk::{prelude::*};

use crate::{layout_config::LayoutConfig, app::AppAction};

pub struct OverlayDetails {
    event_sender: Sender<AppAction>,

    pub container: gtk::Box,
    title: gtk::Label,
    active_state_switch: gtk::Switch,
    
    pub name_entry: gtk::Entry,
    pub url_entry: gtk::Entry,
    pub x_pos_spin: gtk::SpinButton,
    pub y_pos_spin: gtk::SpinButton,
    pub width_spin: gtk::SpinButton,
    pub height_spin: gtk::SpinButton,
    pub clickthrough_check: gtk::CheckButton,
    pub movable_check: gtk::CheckButton,

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
            event_sender: sender,

            container: infos_container,
            title: gtk::Label::default(),
            active_state_switch: gtk::Switch::default(),

            name_entry: gtk::Entry::default(),
            url_entry: gtk::Entry::default(),
            x_pos_spin: OverlayDetails::create_spinbutton(),
            y_pos_spin: OverlayDetails::create_spinbutton(),
            width_spin: OverlayDetails::create_spinbutton(),
            height_spin: OverlayDetails::create_spinbutton(),
            clickthrough_check: gtk::CheckButton::with_label("Clickthrough"),
            movable_check: gtk::CheckButton::with_label("Movable"),
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
    
    fn create_spinbutton() -> gtk::SpinButton {
        let adjustment = gtk::Adjustment::new(0.0, 0.0, 5000.0, 1.0, 1.0, 0.0);
        gtk::SpinButton::new(Some(&adjustment), 1.0, 0)
    }
    
    fn create_header(&mut self) -> gtk::Widget {
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
        header.style_context().add_class("overlay-header");

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
        form_box.add(&self.url_entry);
        form_box.add(&self.x_pos_spin);
        form_box.add(&self.y_pos_spin);
        form_box.add(&self.width_spin);
        form_box.add(&self.height_spin);
        form_box.add(&self.clickthrough_check);
        form_box.add(&self.movable_check);
        form_box.add(&self.save_button);

        form_box.into()
    }

    pub fn set_current_overlay(&mut self, overlay: LayoutConfig) {
        self.disconnect_signals();

        // Update the form entries
        self.title.set_text(&overlay.name());
        self.active_state_switch.set_state(overlay.is_active());

        self.name_entry.set_text(&overlay.name());
        self.url_entry.set_text(&overlay.url());
        self.x_pos_spin.set_value(overlay.x() as f64);
        self.y_pos_spin.set_value(overlay.y() as f64);
        self.width_spin.set_value(overlay.width() as f64);
        self.height_spin.set_value(overlay.height() as f64);
        self.clickthrough_check.set_active(overlay.is_clickthrough());
        self.movable_check.set_active(overlay.is_decoraded());
        
        self.setup_signals(overlay);
    }
    
    fn setup_signals(&mut self, overlay: LayoutConfig) {
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
    
        let event_sender = self.event_sender.clone();
        self.save_handler_id = Some(
            self.save_button.connect_clicked(move |_| {
                let event_sender = event_sender.clone();
                let overlay_cloned = overlay.clone();
                glib::MainContext::default().block_on(async move {
                    let _ = event_sender.send(AppAction::SaveOverlay(overlay_cloned)).await;
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let _ = event_sender.send(AppAction::LoadOverlaysList).await;
                })
            })
        );
    }

    fn disconnect_signals(&mut self) {
        // Important: Disable the events BEFORE assigning new values :)
        if let Some(signal_handler) = self.switch_handler_id.take() {
            self.active_state_switch.disconnect(signal_handler);
        }
        if let Some(signal_handler) = self.save_handler_id.take() {
            self.save_button.disconnect(signal_handler);
        }
    }
}



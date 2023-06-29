use glib::{Sender, SignalHandlerId};
use gtk::{prelude::*};

use crate::{layout_config::LayoutConfig};

use super::OverlaySignals;

pub struct OverlayInfos {
    current_overlay: Option<LayoutConfig>,
    sender: Sender<OverlaySignals>,

    name_entry: gtk::Entry,
    
    active_state_switch: gtk::Switch,
    switch_handler_id: Option<SignalHandlerId>
}

impl OverlayInfos {
    pub fn new(sender: Sender<OverlaySignals>) -> Self {
        let switch = gtk::Switch::builder()
            .build();
        
        Self {
            current_overlay: None,
            sender,

            name_entry: gtk::Entry::new(),
            active_state_switch: switch,
            switch_handler_id: None
        }
    }

    pub fn ui(&mut self) -> gtk::Widget{
        let infos_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin(15)
            .build();
    
        let header = self.create_header();
        let form = self.create_form();
    
        infos_container.add(&header);
        infos_container.add(&form);

        infos_container.into()
    }
    
    fn create_header<'a>(&mut self) -> gtk::Widget {
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
        
        let default_label = if let Some(overlay) = &self.current_overlay {
            overlay.name()
        } else {
            "".to_owned()
        }; 
        let title_text = gtk::Label::builder()
            .label(&default_label)
            .build();
        title_text
            .style_context()
            .add_class("overlay-title");
        
        let cloned_config = self.current_overlay.clone();
        let cloned_sender = self.sender.clone();
        
        if let Some(handler_id) = self.switch_handler_id.take() {
            println!("Disconnect");
            self.active_state_switch.disconnect(handler_id);
        }
        self.switch_handler_id = Some(self.active_state_switch.connect_state_set(move |_, new_state| {
            if let Some(overlay) = &cloned_config {
                let signal = OverlaySignals::ChangeActiveState(new_state, overlay.clone());
                cloned_sender.send(signal).unwrap();
            }
            Inhibit(true)
        }));
    
        header.add(&title_text);
        header.pack_end(&self.active_state_switch, false, false, 0);

        header.into()
    }

    fn create_form(&self) -> gtk::Widget {
        let form_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        form_box.add(&self.name_entry);
        self.name_entry.connect_changed(move |entry| {
            println!("New name: {}", entry.text());
        });

        form_box.into()
    }

    pub fn is_current_overlay(&self, overlay: &LayoutConfig) -> bool {
        if let Some(current_overlay) = &self.current_overlay {
            return current_overlay.clone() == overlay.clone();
        }
        false
    }

    pub fn set_current_overlay(&mut self, overlay: LayoutConfig) {
        // Update the form entries
        self.name_entry.set_text(&overlay.name());

        self.current_overlay = Some(overlay);
    }
}



use glib::{Sender, subclass::{types::ObjectSubclass, prelude::ObjectImpl}};
use gtk::{prelude::*};

use crate::{layout_config::LayoutConfig};

use super::OverlaySignals;

pub struct OverlayInfos {
    current_overlay: LayoutConfig,
    sender: Sender<OverlaySignals>,
    active_state_switch: gtk::Switch
}

impl OverlayInfos {
    pub fn new(layout_config: LayoutConfig, sender: Sender<OverlaySignals>) -> Self {
        let switch = gtk::Switch::builder()
            .build();
        
        Self {
            current_overlay: layout_config,
            active_state_switch: switch,
            sender
        }
    }

    pub fn ui(&self) -> gtk::Widget{
        println!("New display {:?}", self.current_overlay);
        let infos_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin(15)
            .build();
    
        let header = self.create_header();     
    
        infos_container.add(&header);
        infos_container.into()
    }
    
    fn create_header<'a>(&self) -> gtk::Widget {
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
    
        let title_text = gtk::Label::builder()
            .label(&self.current_overlay.name())
            .build();
        title_text
            .style_context()
            .add_class("overlay-title");
    
        
        let cloned_config = self.current_overlay.clone();
        let cloned_sender = self.sender.clone();
        self.active_state_switch.connect_state_set(move |_, new_state| {
            let signal = OverlaySignals::ChangeActiveState(new_state, cloned_config.clone());
            cloned_sender.send(signal).unwrap();
            Inhibit(true)
        });
    
        header.add(&title_text);
        header.pack_end(&self.active_state_switch, false, false, 0);

        header.into()
    }

    pub fn is_current_overlay(&self, overlay: &LayoutConfig) -> bool {
        self.current_overlay == overlay.clone()
    }

    pub fn set_current_overlay(&mut self, overlay: LayoutConfig) {
        self.current_overlay = overlay;

        // TODO: Update the bound struct
    }
}



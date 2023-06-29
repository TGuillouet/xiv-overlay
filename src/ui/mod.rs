use async_channel::Sender;
use gtk::traits::PanedExt;

use crate::{layout_config::LayoutConfig, app::AppAction};

use self::sidebar::Sidebar;

pub mod overlay_infos;
pub mod sidebar;

pub enum OverlaySignals {
    ChangeActiveState(bool, LayoutConfig),
    Save(LayoutConfig),
    Remove(LayoutConfig),
}

pub struct AppContainer {
    pub container: gtk::Paned,
    pub sidebar: Sidebar
}

impl AppContainer {
    pub fn new(event_sender: Sender<AppAction>) -> Self{
        let container = gtk::Paned::new(gtk::Orientation::Horizontal);

        let sidebar = Sidebar::new(event_sender.clone());
        let overlay_infos_frame = gtk::Frame::new(None);
        
        container.pack1(&sidebar.frame, false, false);
        container.pack2(&overlay_infos_frame, true, true);

        let app_container = Self {
            container,
            sidebar
        };

        app_container
    }
}


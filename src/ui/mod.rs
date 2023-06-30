use async_channel::Sender;
use gtk::traits::{PanedExt, WidgetExt, ContainerExt};

use crate::{layout_config::LayoutConfig, app::AppAction};

use self::{sidebar::Sidebar, overlay_infos::OverlayDetails};

pub mod overlay_infos;
pub mod sidebar;

pub enum OverlaySignals {
    ChangeActiveState(bool, LayoutConfig),
    Save(LayoutConfig),
    Remove(LayoutConfig),
}

pub struct AppContainer {
    pub container: gtk::Paned,
    pub sidebar: Sidebar,
    pub overlay_details: OverlayDetails
}

impl AppContainer {
    pub fn new(event_sender: Sender<AppAction>) -> Self{
        let container = gtk::Paned::new(gtk::Orientation::Horizontal);

        let sidebar = Sidebar::new(event_sender.clone());
        let overlay_details = OverlayDetails::new(event_sender.clone());
        
        container.pack1(&sidebar.frame, false, false);
        container.pack2(&overlay_details.container, true, true);

        let app_container = Self {
            container,
            sidebar,
            overlay_details
        };

        app_container
    }

    pub fn set_details_visible(&self, is_visible: bool) {
        for child in self.overlay_details.container.children() {
            if is_visible {
                child.show()
            } else {
                child.hide();
            }
        }
    }
}


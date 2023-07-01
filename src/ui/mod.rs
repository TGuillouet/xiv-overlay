use async_channel::Sender;
use gtk::traits::{PanedExt, WidgetExt, ContainerExt};

use crate::app::AppAction;

use self::{sidebar::Sidebar, overlay_infos::OverlayDetails};

pub mod overlay_infos;
pub mod sidebar;

pub struct AppContainer {
    pub container: gtk::Paned,
    pub sidebar: Sidebar,
    pub overlay_details: OverlayDetails
}

impl AppContainer {
    pub fn new(event_sender: Sender<AppAction>) -> Self{
        let container = gtk::Paned::new(gtk::Orientation::Horizontal);

        let sidebar = Sidebar::new(event_sender.clone());
        let overlay_details = OverlayDetails::new(event_sender);
        
        container.pack1(&sidebar.frame, false, false);
        container.pack2(&overlay_details.container, true, true);

        Self {
            container,
            sidebar,
            overlay_details
        }
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


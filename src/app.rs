use std::sync::Arc;

use async_channel::Sender;
use gtk::{traits::{WidgetExt, ContainerExt}, Inhibit};

use crate::{app_config::{AppConfig}, layout_config::{LayoutConfig, load_layouts}, ui::AppContainer};

pub enum AppAction {
    LoadOverlaysList,
    DisplayOverlay(LayoutConfig),
}

pub struct WindowState {
    pub event_sender: Sender<AppAction>
}

pub struct App {
    config: Arc<AppConfig>,
    window: gtk::Window,
    app_container: AppContainer,
    state: WindowState
}

impl App {
    pub fn new(app_config: AppConfig, sender: Sender<AppAction>) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_size_request(1000, 700);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let state = WindowState {
            event_sender: sender.clone()
        };

        let app_container = AppContainer::new(sender.clone());

        window.add(&app_container.container);

        let app = Self { 
            config: Arc::new(app_config),
            window,
            app_container,
            state
        };

        app.show();

        app
    }

    fn show(&self) {
        self.window.show_all();
        
        let tx = self.state.event_sender.clone();
        glib::MainContext::default().spawn_local(async move {
            tx.send(AppAction::LoadOverlaysList).await;
        });
    }

    pub fn load_overlays_list(&self) {
        println!("Loading the overlays list");
        let overlays = load_layouts();
        self.app_container.sidebar.display_overlays_list(overlays);
    }

    pub fn display_overlay_details(&self, overlay: LayoutConfig) {
        println!("Displaying the overlay {:?}", overlay);
    }
}

// pub fn init(&mut self) {
//     let (sender, receiver) = MainContext::channel(Priority::default());
//     self.sender = Some(sender.clone());

//     let mut win_senders_map = HashMap::<String, Sender<bool>>::new();
//     receiver.attach(None, move |signal| {
//         match signal {
//             OverlaySignals::ChangeActiveState(new_state, overlay_config) => {
//                 if new_state {
//                     println!("Activate the overlay: {}", overlay_config.name());
//                     let (win_sender, win_receiver) = MainContext::channel(Priority::default());
//                     let cloned_config = overlay_config.clone();

//                     win_senders_map.insert(overlay_config.name(), win_sender.clone());
//                     std::thread::spawn(move || {
//                         glib::MainContext::default().invoke(move || {
//                             show_overlay(&cloned_config.clone(), win_receiver);
//                         });
//                         0
//                     });
//                 } else {
//                     println!("Disable the overlay: {}", overlay_config.name());
//                     if let Some(sender) = win_senders_map.get(&overlay_config.name()) {
//                         sender.send(true).unwrap();
//                     }
//                 }
//             },
//             _ => {}
//         }

//         glib::Continue(true)
//     });
// }

// pub fn show(&self) {
//     let (selection_sender, selection_receiver) = MainContext::channel(Priority::default());
    
//     let window = Window::new(WindowType::Toplevel);
//     window.set_size_request(1000, 700);

//     let layout = gtk::Paned::new(gtk::Orientation::Horizontal);
//     let mut sidebar = Sidebar::new(selection_sender);
//     let app_content_frame = gtk::Frame::new(None);

//     layout.pack1(&sidebar.ui(), false, false);
//     layout.pack2(&app_content_frame, true, false);

//     let overlay_widget = Arc::new(Mutex::new(OverlayInfos::new(
//         self.sender.clone().unwrap()
//     )));

//     window.add(&layout);

//     window.show_all();

//     window.connect_delete_event(|_, _| {
//         gtk::main_quit();
//         Inhibit(false)
//     });

//     let overlay_clone = Arc::clone(&overlay_widget);
//     selection_receiver.attach(None, move |new_overlay: LayoutConfig| {
//         let mut overlay = overlay_clone.lock().unwrap();
        
//         if overlay.is_current_overlay(&new_overlay) {
//             return glib::Continue(true);
//         }

//         println!("New displayed overlay {}", new_overlay.name());

//         for children in app_content_frame.children().iter() {
//             app_content_frame.remove(children);
//         }

//         overlay.set_current_overlay(new_overlay);
//         app_content_frame.add(&overlay.ui());
//         app_content_frame.show_all();

//         glib::Continue(true)
//     });
// }

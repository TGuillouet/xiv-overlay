use crate::layout_config::LayoutConfig;

use glib::Receiver;
use gtk::prelude::ContainerExt;
use gtk::traits::GtkWindowExt;
use gtk::{Inhibit, Window, WindowType, traits::WidgetExt};
use gdk::RGBA;
use webkit2gtk::{
    traits::{SettingsExt, WebViewExt},
    WebContext, WebView,
};

fn set_visual(window: &gtk::Window, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = GtkWindowExt::screen(window) {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

pub fn show_overlay(config: &LayoutConfig, shutdown_receiver: Receiver<bool>) {
    let window = Window::new(WindowType::Toplevel);
    set_visual(&window, None);

    window.set_app_paintable(true);
    window.set_decorated(config.is_decoraded());
    window.set_keep_above(true);
    window.set_size_request(
        config.width(), 
        config.height()
    );
    window.move_(config.x(), config.y());

    window.connect_screen_changed(set_visual);
    window.connect_draw(|_window, ctx| {
        ctx.set_source_rgba(1.0, 0.0, 0.0, 0.0);
        ctx.set_operator(cairo::Operator::Screen);
        let _ = ctx.paint();
        Inhibit(false)
    });

    let context = WebContext::default().unwrap();

    let webview = WebView::with_context(&context);
    webview.load_uri(&config.url());
    webview.set_background_color(&RGBA::new(0.0, 0.0, 0.0, 0.0));
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);
    
    window.show_all();
    
    if config.is_clickthrough() {
        let gdk_window = window.window()
            .expect("Could not fetch the gdk window");
        let region = cairo::Region::create();
        gdk_window.input_shape_combine_region(&region, 0, 0);
    }

    shutdown_receiver.attach(None, move |_| {
        window.close();

        glib::Continue(true)
    });
}
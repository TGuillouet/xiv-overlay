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

fn main() {
    // TODO: Parse the configuration

    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    set_visual(&window, None);

    window.set_app_paintable(true);

    window.connect_screen_changed(set_visual);
    window.connect_draw(|_window, ctx| {
        ctx.set_source_rgba(1.0, 0.0, 0.0, 0.0);
        ctx.set_operator(cairo::Operator::Screen);
        let _ = ctx.paint();
        Inhibit(false)
    });

    // Display or not the title bar
    // window.set_decorated(false);

    // TODO: Clickthrough (with cairo)

    let context = WebContext::default().unwrap();

    let webview = WebView::with_context(&context);
    webview.load_uri("http://proxy.iinact.com/overlay/skyline/?OVERLAY_WS=ws://127.0.0.1:10501/ws");
    webview.set_background_color(&RGBA::new(0.0, 0.0, 0.0, 0.0));
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    /*let inspector = webview.get_inspector().unwrap();
    inspector.show();*/
    
    window.show_all();
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });



    gtk::main();
}

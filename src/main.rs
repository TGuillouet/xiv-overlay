use gtk::prelude::ContainerExt;
use gtk::{Inhibit, Window, WindowType, traits::WidgetExt};
use gdk::RGBA;
use webkit2gtk::{
    traits::{SettingsExt, WebContextExt, WebViewExt},
    WebContext, WebView,
};

fn set_visual(window: &gtk::Window, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn main() {
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

    let context = WebContext::default().unwrap();
    context.set_web_extensions_directory("../webkit2gtk-webextension-rs/example/target/debug/");

    #[cfg(not(feature = "v2_6"))]
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

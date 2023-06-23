mod config;

use config::Config;
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
    let config = Config::from_file("./config.yaml")
        .expect("Could not parse the configuration");
    println!("{:?}", config);
    
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    set_visual(&window, None);

    window.set_app_paintable(true);
    window.set_decorated(config.is_decoraded());

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

    /*let inspector = webview.get_inspector().unwrap();
    inspector.show();*/
    
    window.show_all();
    
    let gdk_window = window.window()
        .expect("Could not fetch the gdk window");
    gdk_window.move_(
        config.x(), 
        config.y(), 
    );
    gdk_window.resize(
        config.width(), 
        config.height()
    );
        
    if config.is_clickthrough() {
        let region = cairo::Region::create();
        gdk_window.input_shape_combine_region(&region, 0, 0);
    }

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

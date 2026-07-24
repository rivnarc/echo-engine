use dioxus::prelude::*;
use dioxus::desktop::{
    Config, 
    WindowBuilder, 
    LogicalSize,
    tao::window::Icon,
};

static ICON_BYTES: &[u8] = include_bytes!("../../../assets/logo.png");
static HTML: &str = include_str!("web/index.html");
static CSS: &str = include_str!("web/styles.css");

pub fn launcher() {
    unsafe {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }
    
    let icon = load_icon();

    let window = WindowBuilder::new()
        .with_title("Мой Лаунчер")
        .with_inner_size(LogicalSize::new(1000.0, 600.0))
        .with_resizable(true)
        .with_window_icon(Some(icon));

    let config = Config::new().with_window(window);
    LaunchBuilder::desktop().with_cfg(config).launch(app);
}

fn app() -> Element {
    rsx! {
        style { "{CSS}" }

        div { dangerous_inner_html: "{HTML}" }
    }
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_BYTES)
            .expect("Не удалось загрузить иконку")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Не удалось создать Icon")
}
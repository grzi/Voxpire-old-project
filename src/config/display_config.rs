use amethyst::{
    window::{DisplayConfig, MonitorIdent},
    winit::EventsLoop,
};

fn create_config(fullscreen: Option<MonitorIdent>) -> DisplayConfig {
    DisplayConfig {
        title: "Rumple's adventures".to_string(),
        fullscreen,
        dimensions: Some((900, 900)),
        min_dimensions: Some((500, 500)),
        max_dimensions: None,
        visibility: true,
        icon: None,
        always_on_top: false,
        decorations: true,
        maximized: false,
        multitouch: false,
        resizable: true,
        transparent: false,
        loaded_icon: None,
    }
}

pub fn windowed() -> DisplayConfig {
    _full_screen()
}

pub fn _full_screen() -> DisplayConfig {
    let el = EventsLoop::new();
    create_config(Some(MonitorIdent::from_primary(&el)))
}

use amethyst::config::Config;
use amethyst::input::{Bindings, StringBindings};
use amethyst::utils::application_root_dir;

use log::error;

pub(crate) fn default() -> Bindings<StringBindings> {
    let default_bindings = application_root_dir()
        .unwrap()
        .join("assets")
        .join("config")
        .join("default_bindings.ron");
    let bindings = Bindings::load(default_bindings);
    bindings.unwrap_or_else(|e| {
        error!("Can't read default input bindings file {}", e.to_string());
        Bindings::new()
    })
}

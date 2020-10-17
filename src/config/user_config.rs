use crate::config::{bindings_config, display_config, logging_config};
use crate::utilities::data_access;
use amethyst::input::{Bindings, StringBindings};
use amethyst::{window::DisplayConfig, LoggerConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub logger_config: LoggerConfig,
    pub display_config: DisplayConfig,
    pub bindings_config: Bindings<StringBindings>,
}

impl UserConfig {
    fn new() -> UserConfig {
        UserConfig {
            logger_config: logging_config::create(),
            display_config: display_config::windowed(),
            bindings_config: bindings_config::default(),
        }
    }
}

pub fn retrieve_user_config() -> UserConfig {
    match data_access::read_and_deserialize::<UserConfig>("user_config.ron") {
        None => {
            let new_user_config = UserConfig::new();
            if data_access::save_to_file(&new_user_config, "user_config.ron").is_err() {
                println!("Impossible to save user configuration...");
            }
            new_user_config
        }
        Some(config) => config,
    }
}

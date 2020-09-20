use crate::config::display_config;
use crate::config::logging_config;
use crate::utilities::data_access;
use amethyst::window::DisplayConfig;
use amethyst::LoggerConfig;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub logger_config: LoggerConfig,
    pub display_config: DisplayConfig,
}

impl UserConfig {
    fn new() -> UserConfig {
        UserConfig {
            logger_config: logging_config::create(),
            display_config: display_config::windowed(),
        }
    }
}

pub fn retrieve_user_config() -> UserConfig {
    match data_access::read_and_deserialize::<UserConfig>("user_config.ron") {
        None => {
            let new_user_config = UserConfig::new();
            match data_access::save_to_file(&new_user_config, "user_config.ron") {
                Err(_e) => println!("Impossible to save user configuration..."),
                _ => {}
            }
            new_user_config
        }
        Some(config) => config,
    }
}

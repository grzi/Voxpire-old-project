mod config;
mod entities;
mod systems;
mod utilities;

use amethyst::{Result, start_logger, utils::application_root_dir, GameDataBuilder, Application};
use amethyst::renderer::{RenderingBundle, RenderToWindow, types::DefaultBackend};
use crate::entities::main_state::MainState;

fn main() -> Result<()> {
    let user_config = config::user_config::retrieve_user_config();
    start_logger(user_config.logger_config);

    let game_data = GameDataBuilder::default()
       .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(user_config.display_config)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
        )?
        .with(systems::config_system::ConfigSystem, "config_system", &[]);

    Application::new(application_root_dir()?.join("assets"), MainState, game_data)?
        .run();
    Ok(())
}

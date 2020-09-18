mod config;
mod entities;

use amethyst::{Result, start_logger, utils::application_root_dir, GameDataBuilder, Application};
use amethyst::renderer::{RenderingBundle, RenderToWindow, types::DefaultBackend};
use crate::entities::main_state::MainState;

fn main() -> Result<()> {
    start_logger(config::logging_development_config());

    let game_data = GameDataBuilder::default()
       .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(config::display_config::windowed())
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
        )?;
    Application::new(application_root_dir()?.join("assets"), MainState, game_data)?
        .run();
    Ok(())
}

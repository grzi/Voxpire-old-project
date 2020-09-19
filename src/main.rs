mod config;
mod states;
mod systems;
mod utilities;

use amethyst::{Result, start_logger, utils::application_root_dir, GameDataBuilder, Application, assets::PrefabLoaderSystemDesc};
use amethyst::renderer::{RenderingBundle, RenderToWindow, types::DefaultBackend, RenderShaded3D };
use crate::states::main_state::{MainState, MyPrefabData};
use amethyst::core::TransformBundle;
use amethyst::input::{StringBindings, InputBundle};
use amethyst::ui::{RenderUi, UiBundle};
use crate::systems::ui_systems::UiSystemEventHandlerSystemDesc;

fn main() -> Result<()> {
    let user_config = config::user_config::retrieve_user_config();
    start_logger(user_config.logger_config);

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_system_desc(UiSystemEventHandlerSystemDesc::default(), "ui_handler", &[])
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(user_config.display_config)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_thread_local(systems::config_system::ConfigSystem);

    Application::new(application_root_dir()?.join("assets"), MainState, game_data)?
        .run();
    Ok(())
}

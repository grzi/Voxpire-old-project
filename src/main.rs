mod config;
mod resources;
mod states;
mod systems;
mod utilities;

use crate::states::main_state::{MainState, MyPrefabData};
use amethyst::{assets::PrefabLoaderSystemDesc, core::TransformBundle, input::{InputBundle, StringBindings}, renderer::{types::DefaultBackend, RenderShaded3D, RenderToWindow, RenderingBundle}, start_logger, ui::{RenderUi, UiBundle}, utils::application_root_dir, Application, GameDataBuilder, Result};
use amethyst_developer_console::developer_console::DeveloperConsoleSystem;
use crate::systems::debug_system::DebugSystem;

fn main() -> Result<()> {
    let user_config = config::user_config::retrieve_user_config();
    start_logger(user_config.logger_config);
    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with(DeveloperConsoleSystem::new(), "developer-console", &[])
        .with(DebugSystem, "debug_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings(user_config.bindings_config),
        )?
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

    Application::new(application_root_dir()?.join("assets"), MainState, game_data)?.run();
    Ok(())
}

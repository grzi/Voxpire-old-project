mod config;
mod states;
mod ui;
mod utilities;
mod engines;

use crate::states::main_state::{MainState, MyPrefabData};
use amethyst::{assets::PrefabLoaderSystemDesc, core::TransformBundle, input::{InputBundle, StringBindings}, renderer::{types::DefaultBackend, RenderShaded3D, RenderToWindow, RenderingBundle}, start_logger, ui::{RenderUi, UiBundle}, utils::application_root_dir, Application, GameDataBuilder, Result};
use amethyst_developer_console::developer_console::DeveloperConsoleSystem;
use crate::states::CurrentState;
use amethyst::core::SystemExt;
use crate::engines::time::time_system::TimeSystem;
use crate::engines::debug::debug_system::DebugSystem;
use amethyst::renderer::RenderSkybox;
use amethyst::renderer::palette::Srgb;

fn main() -> Result<()> {
    let user_config = config::user_config::retrieve_user_config();
    start_logger(user_config.logger_config);
    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with(DeveloperConsoleSystem::new().pausable(CurrentState::InGame), "developer-console", &[])
        .with(DebugSystem.pausable(CurrentState::InGame), "debug_system", &[])
        .with(TimeSystem.pausable(CurrentState::InGame), "time_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings(user_config.bindings_config),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config(user_config.display_config))
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.18, 0.11, 0.85),
                )),
        )?
        .with_thread_local(config::config_system::ConfigSystem);

    Application::new(application_root_dir()?.join("assets"), MainState, game_data)?.run();
    Ok(())
}

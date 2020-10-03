use crate::states::main_menu_state::MainMenuState;
use amethyst::{
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    utils::scene::BasicScenePrefab,
    {GameData, SimpleState, SimpleTrans, StateData, Trans},
};
use log::debug;
use amethyst::core::ecs::WorldExt;
use crate::utilities::developer::developer_console::DeveloperConsoleResource;

pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        _data.world.insert(DeveloperConsoleResource::new());
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        //debug!("Update main state");
        Trans::Switch(Box::new(MainMenuState::default()))
    }
}

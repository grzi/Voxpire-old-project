use crate::states::main_menu_state::MainMenuState;
use amethyst::{
    {GameData, SimpleState, SimpleTrans, StateData, Trans},
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    utils::scene::BasicScenePrefab
};
use log::debug;


pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        debug!("Update main state");
        Trans::Switch(Box::new(MainMenuState::default()))
    }
}

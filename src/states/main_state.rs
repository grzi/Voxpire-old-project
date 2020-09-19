use amethyst::{SimpleState, StateData, GameData, SimpleTrans, Trans};
use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::renderer::rendy::mesh::{Position, Normal, TexCoord};
use amethyst::core::ecs::{WorldExt, Builder};
use amethyst::ui::{UiCreator, UiTransform, UiButton};
use amethyst::winit::DeviceEvent::Button;
use crate::states::main_menu_state::MainMenuState;
use log::debug;
pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {

    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        debug!("Update main state");
        Trans::Switch(Box::new(MainMenuState::default()))
    }
}
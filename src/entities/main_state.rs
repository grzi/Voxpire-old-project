use amethyst::{SimpleState, StateData, GameData};
use log::debug;
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Starting Main State.");
    }
}
pub mod main_menu_state;
pub mod main_state;
pub mod poc_state;

#[derive(PartialEq)]
pub enum CurrentState {
    Main,
    MainMenu,
    InGame,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Main
    }
}

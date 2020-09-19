use amethyst::{SimpleState, StateData, GameData, SimpleTrans, Trans, StateEvent};

use log::{debug, info};
use amethyst::core::ecs::{Entity, World, WorldExt, Builder};
use amethyst::ui::{UiTransform, Anchor, ScaleMode, UiButton, UiText, FontHandle, Interactable, FontAsset, LineMode, UiEventType};
use amethyst::assets::{Loader, AssetStorage};
use crate::states::main_state::MainState;
use amethyst::winit::{Window, EventsLoop};

const QUIT_BUTTON: &str = "QUIT_BUTTON";
const SETTINGS_BUTTON: &str = "SETTINGS_BUTTON";

#[derive(Default)]
pub struct MainMenuState {
    new_game_button: Option<Entity>,
    settings_button: Option<Entity>,
    quit_button: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Launching main menu state");
        let world = data.world;

        self.quit_button = init_quit_button(world);
        self.settings_button = init_settings_button(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.entities_mut().delete(self.quit_button.unwrap());
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = event {
            let is_quit = ui_event.target.id() == self.quit_button.unwrap().id();
            let is_settings = ui_event.target.id() == self.settings_button.unwrap().id();
            if is_quit && ui_event.event_type == UiEventType::Click{
               return Trans::Quit
            }
            if is_settings && ui_event.event_type == UiEventType::Click{
                let mut test = _data.world.fetch_mut::<Window>();
                let el = EventsLoop::new();
                if let None = test.get_fullscreen() {
                    test.set_fullscreen(Some(el.get_primary_monitor()));
                }else {
                    test.set_fullscreen(None);
                }

            }
        }
        Trans::None
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        debug!("Update main menu state");
        Trans::None
    }
}

fn init_quit_button(world: &mut World) -> Option<Entity> {
    let mut ui_transform = UiTransform::new(QUIT_BUTTON.to_string(), Anchor::BottomLeft, Anchor::BottomLeft, 0.4, 0.1, 3., 0.2, 0.05);
    ui_transform.scale_mode = ScaleMode::Percent;
    let ui_text = UiText::new(
        amethyst::ui::get_default_font(&world.read_resource::<Loader>(), &world.read_resource::<AssetStorage<FontAsset>>()),
        String::from("Quit"),
        [1.0,1.0,1.0,1.],
        30.,
        LineMode::Single,
        Anchor::Middle
    );

    Some(world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)
        .build())
}

fn init_settings_button(world: &mut World) -> Option<Entity> {
    let mut ui_transform = UiTransform::new(SETTINGS_BUTTON.to_string(), Anchor::BottomLeft, Anchor::BottomLeft, 0.4, 0.16, 3., 0.2, 0.05);
    ui_transform.scale_mode = ScaleMode::Percent;
    let ui_text = UiText::new(
        amethyst::ui::get_default_font(&world.read_resource::<Loader>(), &world.read_resource::<AssetStorage<FontAsset>>()),
        String::from("Settings"),
        [1.0,1.0,1.0,1.],
        30.,
        LineMode::Single,
        Anchor::Middle
    );

    Some(world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)
        .build())
}
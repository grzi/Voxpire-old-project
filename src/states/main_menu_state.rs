use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::ecs::{Builder, Entity, World, WorldExt};
use amethyst::ui::{
    Anchor, FontAsset, Interactable, LineMode, ScaleMode, UiEventType,
    UiImage, UiText, UiTransform,
};
use amethyst::winit::{EventsLoop, MouseCursor, Window};
use log::{debug, info, warn};

const NEW_BUTTON: &str = "NEW_BUTTON";
const SETTINGS_BUTTON: &str = "SETTINGS_BUTTON";
const QUIT_BUTTON: &str = "QUIT_BUTTON";

#[derive(Default)]
pub struct MainMenuState {
    new_game_button: Option<Entity>,
    settings_button: Option<Entity>,
    quit_button: Option<Entity>,
    entity_ids: Option<(u32, u32, u32)>,
}

impl MainMenuState {
    fn unwrap_btn_ids(&self) -> (u32, u32, u32) {
        (
            self.new_game_button.unwrap().id(),
            self.settings_button.unwrap().id(),
            self.quit_button.unwrap().id(),
        )
    }

    fn create_buttons(&mut self, world: &mut World) {
        self.quit_button = init_quit_button(world);
        self.settings_button = init_settings_button(world);
        self.new_game_button = init_new_button(world);
        self.entity_ids = Some(self.unwrap_btn_ids());
    }

    fn delete_buttons(&self, world: &World) -> bool{
        let entities = world.entities_mut();
        entities.delete(self.new_game_button.unwrap()).is_ok() &&
        entities.delete(self.settings_button.unwrap()).is_ok() &&
        entities.delete(self.quit_button.unwrap()).is_ok()
    }
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Launching main menu state");
        let world = data.world;
        self.create_buttons(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.delete_buttons(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = event {
            let (_new, _settings, _quit) = self.entity_ids.unwrap();
            let buttons_ids = [_new, _settings, _quit];
            match (ui_event.target.id(), ui_event.event_type) {
                (target, ev_type)
                    if (buttons_ids.contains(&target)
                        && (UiEventType::HoverStart == ev_type
                            || UiEventType::HoverStop == ev_type)) => {
                                    handle_hover_event(&data, target, &ev_type)
                }
                (_a, _b) if (_a == _new && _b == UiEventType::Click) => {
                    // TODO : Trans to game
                }
                (_a, _b) if (_a == _settings && _b == UiEventType::Click) => {
                    let world = data.world;
                    {
                        let window = world.fetch_mut::<Window>();
                        if let None = window.get_fullscreen() {
                            let el = EventsLoop::new();
                            window.set_fullscreen(Some(el.get_primary_monitor()));
                        } else {
                            window.set_fullscreen(None);
                        }
                    }
                }
                (_a, _b) if (_a == _quit && _b == UiEventType::Click) => {
                    return Trans::Quit;
                }
                _ => {}
            }
        }
        Trans::None
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        debug!("Update main menu state");
        Trans::None
    }
}

fn to_cursor(ev_type: &UiEventType) -> MouseCursor {
    if &UiEventType::HoverStart == ev_type {
        return MouseCursor::Hand;
    }
    return MouseCursor::Default;
}

fn to_btn_color(ev_type: &UiEventType) -> UiImage {
    if &UiEventType::HoverStart == ev_type {
        return UiImage::SolidColor([0.3, 0.3, 0.3, 0.5]);
    }
    UiImage::SolidColor([0.1, 0.1, 0.1, 0.5])
}

fn init_new_button(world: &mut World) -> Option<Entity> {
    init_button(
        world,
        NEW_BUTTON.to_string(),
        "New game".to_string(),
        (0.4, 0.22, 3.),
    )
}

fn init_settings_button(world: &mut World) -> Option<Entity> {
    init_button(
        world,
        SETTINGS_BUTTON.to_string(),
        "Full Screen Toggle".to_string(),
        (0.4, 0.16, 3.),
    )
}

fn init_quit_button(world: &mut World) -> Option<Entity> {
    init_button(
        world,
        QUIT_BUTTON.to_string(),
        "Quit".to_string(),
        (0.4, 0.1, 3.),
    )
}

fn init_button(
    world: &mut World,
    button_name: String,
    button_text: String,
    (x, y, z): (f32, f32, f32),
) -> Option<Entity> {
    let mut ui_transform = UiTransform::new(
        button_name,
        Anchor::BottomLeft,
        Anchor::BottomLeft,
        x,
        y,
        z,
        0.2,
        0.05,
    );
    ui_transform.scale_mode = ScaleMode::Percent;

    let ui_text = UiText::new(
        amethyst::ui::get_default_font(
            &world.read_resource::<Loader>(),
            &world.read_resource::<AssetStorage<FontAsset>>(),
        ),
        button_text,
        [1.0, 1.0, 1.0, 1.],
        30.,
        LineMode::Single,
        Anchor::Middle,
    );

    Some(
        world
            .create_entity()
            .with(ui_transform)
            .with(ui_text)
            .with(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]))
            .with(Interactable)
            .build(),
    )
}

fn handle_hover_event(data: &StateData<GameData>, target: u32, ev_type: &UiEventType) {
    let mut storage = data.world.write_storage::<UiImage>();
    let current_button_entity = data.world.entities().entity(target);
    storage.remove(current_button_entity);
    if ! storage.insert(current_button_entity, to_btn_color(&ev_type)).is_ok() {
        warn!("Error while trying to add UiImage to storage entity {:?} btn {:?}", current_button_entity, to_btn_color(&ev_type));
    }
    data.world
        .fetch_mut::<Window>()
        .set_cursor(to_cursor(&ev_type));
}

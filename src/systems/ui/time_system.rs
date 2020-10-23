use amethyst::core::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use crate::resources::ui::time::TimeResource;
use amethyst::ui::UiText;
use crate::components::ui::time::TimeComponent;

pub struct TimeSystem;

impl<'s> System<'s> for TimeSystem {
    type SystemData = (
        Read<'s, TimeResource>,
        ReadStorage<'s, TimeComponent>,
        WriteStorage<'s, UiText>
    );

    fn run(&mut self, (time_data, time_component, mut ui_texts): Self::SystemData) {
        for (_component, ui) in (&time_component, &mut ui_texts).join() {
            ui.text = time_data.format();
        }
    }
}
use amethyst::core::shrev::ReaderId;
use amethyst::ui::{UiEvent, UiButton, UiText, UiButtonData, Widgets, UiTransform};
use amethyst::core::ecs::prelude::{Entity, System, SystemData, WorldExt, Write};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::derive::SystemDesc;
use log::info;
use amethyst::core::ecs::{Read, ReadExpect, ReadStorage};

#[derive(SystemDesc)]
#[system_desc(name(UiSystemEventHandlerSystemDesc))]
pub struct UiSystemEventHandler {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
}

impl UiSystemEventHandler {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'a> System<'a> for UiSystemEventHandler {
    type SystemData = (Write<'a, EventChannel<UiEvent>>,ReadStorage<'a, UiTransform> ,ReadExpect<'a, Widgets<UiButton>>);

    fn run(&mut self, (events, transforms, texts): Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            match transforms.get(ev.target) {
                Some(button) => {},
                _ => {}
            }

        }
    }
}
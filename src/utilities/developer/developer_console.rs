use std::collections::VecDeque;
use amethyst::core::ecs::{System, SystemData, Read, Write, Entities, WriteStorage, ReadExpect, Join};
use amethyst::input::{StringBindings, InputHandler};
use log::info;
use amethyst::derive::SystemDesc;
use amethyst::ui::{UiTransform, Anchor, ScaleMode, UiText, UiImage, TextEditing, FontHandle, FontAsset, Interactable, Selectable};
use amethyst::assets::{Loader, AssetStorage};
use crate::utilities::developer::developer_console_utils;

pub const OPEN_DEVELOPER_CONSOLE_ACTION: &str = "open-developer-console";

pub enum HistoryLine {
    Command(String),
    Result(Vec::<String>)
}

pub struct DeveloperConsoleResource {
    pub font_handle: Option<FontHandle>,
    pub displayed: bool,
    pub history : Vec::<HistoryLine>,
    pub command_queue: VecDeque::<String>
}

impl Default for DeveloperConsoleResource{
    fn default() -> Self {
        DeveloperConsoleResource::new()
    }
}

impl DeveloperConsoleResource {
    pub fn new () -> Self {
        DeveloperConsoleResource{
            font_handle: None,
            displayed: false,
            history: Vec::new(),
            command_queue: VecDeque::new()
        }
    }

    pub fn add_line(&mut self, line: HistoryLine) {
        if let HistoryLine::Command(command) = &line {
            self.command_queue.push_back(command.to_string());
        }
        self.history.push(line);
    }

    pub fn read_last_command(&mut self) -> Option<String> {
        self.command_queue.pop_front()
    }
}

#[derive(SystemDesc)]
pub struct DeveloperConsoleSystem {
    action_pressed: bool,
    console_displayed: bool
}

impl DeveloperConsoleSystem {
    pub fn new() -> DeveloperConsoleSystem {
        DeveloperConsoleSystem {
            action_pressed: false,
            console_displayed: false
        }
    }

    fn toggle_console(&mut self,
                      mut developer_resource: Write<DeveloperConsoleResource>,
                      loader: ReadExpect<Loader>,
                      font_asset: Read<AssetStorage<FontAsset>>,
                      mut entities: Entities,
                      mut ui_transform: WriteStorage<UiTransform>,
                      mut interactable_storage: WriteStorage<Interactable>,
                      mut selectable_storage: WriteStorage<Selectable<()>>,
                      mut ui_image: WriteStorage<UiImage>,
                      mut ui_texts: WriteStorage<UiText>,
                      mut ui_text_editings: WriteStorage<TextEditing>) {
        if !developer_resource.displayed { // TOGGLE using resource flag
            // TODO :
            //      Add a console font
            //      Real toggle
            //      display content of resource in a text area
            //      Catch enter from text_editing
            entities.build_entity()
                .with(developer_console_utils::create_input_transform(), &mut ui_transform)
                .with(developer_console_utils::create_ui_text(&developer_resource, loader, font_asset), &mut ui_texts)
                .with(developer_console_utils::create_text_editing(), &mut ui_text_editings)
                .with(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]), &mut ui_image)
                .with(Interactable, &mut interactable_storage)
                .with(Selectable::<()>::new(0), &mut selectable_storage)
                .build();
            developer_resource.displayed = true;
        } else {
            let mut console_entity = None;
            for (entity, transform) in (&*entities, &ui_transform).join() {
                if transform.id == String::from("developer-console-input-transform") {
                    console_entity = Some(entity);
                }
            }
            if let Some(e) = console_entity {
                entities.delete(e);
                developer_resource.displayed = false;
            }
        }
    }
}

impl <'s> System<'s> for DeveloperConsoleSystem {
    type SystemData = (ReadExpect<'s, Loader>,
                       Read<'s, AssetStorage<FontAsset>>,
                       Read<'s, InputHandler<StringBindings>>,
                       Write<'s, DeveloperConsoleResource>,
                       WriteStorage<'s, UiTransform>,
                       WriteStorage<'s, Interactable>,
                       WriteStorage<'s, Selectable<()>>,
                       WriteStorage<'s, UiImage>,
                       WriteStorage<'s, UiText>,
                       WriteStorage<'s, TextEditing>,
                       Entities<'s>);

    fn run(&mut self, (loader, font_assets, input, mut console_resource, mut ui_transform, mut interactables, mut selectables, mut ui_images, mut ui_texts, mut text_editings,mut entities): Self::SystemData) {
        if let Some(true) = input.action_is_down(OPEN_DEVELOPER_CONSOLE_ACTION) {
            if !self.action_pressed {
                self.action_pressed = true;
                self.toggle_console(console_resource, loader, font_assets, entities, ui_transform,interactables, selectables,ui_images, ui_texts, text_editings);
            }
        }else{
            self.action_pressed = false;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_last_command() {
        let mut resource = DeveloperConsoleResource::new();
        assert_eq!(None, resource.read_last_command());
        resource.add_line(HistoryLine::Command(String::from("execute a command")));
        assert_eq!(Some(String::from("execute a command")), resource.read_last_command());
    }
}

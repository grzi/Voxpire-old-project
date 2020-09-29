use std::collections::VecDeque;
use amethyst::core::ecs::{System, SystemData, Read, Write, Entities, WriteStorage};
use amethyst::input::{StringBindings, InputHandler};
use log::info;
use amethyst::derive::SystemDesc;
use amethyst::ui::{UiTransform, Anchor, ScaleMode, UiText, UiImage};
use amethyst::assets::Loader;
use crate::utilities::developer::developer_console_utils;

pub const OPEN_DEVELOPER_CONSOLE_ACTION: &str = "open-developer-console";

pub enum HistoryLine {
    Command(String),
    Result(Vec::<String>)
}

pub struct DeveloperConsoleResource {
    displayed: bool,
    history : Vec::<HistoryLine>,
    command_queue: VecDeque::<String>
}

impl Default for DeveloperConsoleResource{
    fn default() -> Self {
        DeveloperConsoleResource::new()
    }
}

impl DeveloperConsoleResource {
    pub fn new () -> Self {
        DeveloperConsoleResource{
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
                      mut entities: Entities,
                      mut ui_transform: WriteStorage<UiTransform>,
                      mut ui_image: WriteStorage<UiImage>,
                      mut ui_texts: WriteStorage<UiText>,
                      mut ui_text_editings: WriteStorage<UiTextEditing>) {
        if true { // TOGGLE using resource flag
            // TODO :
            //      Create ui text and text editing
            //      Add a console font
            //      Real toggle
            //      display content of resource in a text area
            //      Catch enter from text_editing

            entities.build_entity()
                .with(developer_console_utils::create_input_transform(), &mut ui_transform)
                .with(developer_console_utils::create_ui_text(), &mut ui_texts)
                .with(developer_console_utils::create_text_editing(), &mut ui_text_editings)
                .with(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]), &mut ui_image)
                .build();
        } else {
            // TODO : Delete entity
        }
    }
}

impl <'s> System<'s> for DeveloperConsoleSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, DeveloperConsoleResource>,
                       WriteStorage<'s, UiTransform>,
                       WriteStorage<'s, UiImage>,
                       Entities<'s>);


    fn run(&mut self, (input, mut _console_resource, mut ui_transform, mut ui_images, mut entities): Self::SystemData) {
        if let Some(true) = input.action_is_down(OPEN_DEVELOPER_CONSOLE_ACTION) {
            if !self.action_pressed {
                self.action_pressed = true;
                self.toggle_console(entities, ui_transform, ui_images);
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
    fn test_read_last_command (){
        let mut resource = DeveloperConsoleResource::new();
        assert_eq!(None, resource.read_last_command());
        resource.add_line(HistoryLine::Command(String::from("execute a command")));
        assert_eq!( Some(String::from("execute a command")), resource.read_last_command());
    }
}





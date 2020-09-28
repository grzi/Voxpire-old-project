use std::collections::VecDeque;
use amethyst::core::ecs::{System,SystemData, Read, Write};
use amethyst::input::{StringBindings, InputHandler};
use log::info;
use amethyst::derive::SystemDesc;

pub const OPEN_DEVELOPER_CONSOLE_ACTION: &str = "open-developer-console";

pub enum HistoryLine {
    Command(String),
    Result(Vec::<String>)
}

pub struct DeveloperConsoleResource {
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

    fn toggle_console(&mut self) {

    }
}

impl <'s> System<'s> for DeveloperConsoleSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, DeveloperConsoleResource>,
    );

    fn run(&mut self, (input, mut _console_resource): Self::SystemData) {
        if let Some(true) = input.action_is_down(OPEN_DEVELOPER_CONSOLE_ACTION) {
            if !self.action_pressed {
                self.action_pressed = true;
                self.toggle_console();
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





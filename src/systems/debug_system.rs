use amethyst::core::ecs::{System, Write, ReadStorage, WriteStorage, Join};
use amethyst_developer_console::developer_console::DeveloperConsoleResource;
use log::info;
use amethyst::renderer::Camera;
use amethyst::core::Transform;
use std::num::ParseFloatError;

#[derive(Default)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        Write<'s, DeveloperConsoleResource>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        match data.0.read_last_command() {
            Some(thing) => {
                handle_command(thing, data.1, data.2);
            },
            _ => {}
        };
    }
}

fn handle_command(command: String, cameras : ReadStorage<Camera>, mut transforms: WriteStorage<Transform>) {
    const MOVE_CAMERA: &'static str = "move_camera ";
    match command {
        mut camera_movement if camera_movement.starts_with(MOVE_CAMERA) => {
            let camera_movement = camera_movement.split_off(12);
            let args : Vec<&str> = camera_movement.split(" ").collect();
            if args.len() == 3 {
                for (_, transform) in (&cameras, &mut transforms).join() {
                    if let Ok(result) = convert_params_to_xyz(args.get(0).unwrap(), args.get(1).unwrap(), args.get(2).unwrap()) {
                        let (x, y, z) = result;
                        info!("Moving camera to {} {} {}", x, y, z);
                        transform.set_translation_xyz(x, y, z);
                    }
                }
            }
        },
        _ => {}
    };
}

fn convert_params_to_xyz(x_str: &str, y_str: &str, z_str: &str) -> Result<(f32, f32, f32), ParseFloatError> {
    let x = x_str.parse::<f32>();
    let y = y_str.parse::<f32>();
    let z = z_str.parse::<f32>();

    match (x, y, z) {
        (Err(e),_,_) | (_,Err(e),_) | (_,_,Err(e)) => {
            Err(e)
        },
        (Ok(x), Ok(y), Ok(z)) => { Ok((x, y, z))}
    }
}

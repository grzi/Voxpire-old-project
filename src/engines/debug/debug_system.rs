use amethyst::core::ecs::{System, Write, ReadStorage, WriteStorage, Join};
use amethyst_developer_console::developer_console::DeveloperConsoleResource;
use log::info;
use amethyst::renderer::Camera;
use amethyst::core::Transform;
use std::num::ParseFloatError;



use amethyst::core::math::Vector3;

#[derive(Default)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        Write<'s, DeveloperConsoleResource>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        if let Some(thing) = data.0.read_last_command() {
            handle_command(thing, data.1, data.2);
        };
    }
}

fn handle_command(command: String, cameras : ReadStorage<Camera>, transforms: WriteStorage<Transform>) {
    const MOVE_CAMERA: &str = "move_camera ";
    const FOCUS_CAMERA: &str = "focus_camera ";
    const MOVE_LIGHT: &str = "move_light ";
    match command {
        mut command if command.starts_with(MOVE_CAMERA) => move_camera(cameras, transforms, &mut command),
        mut command if command.starts_with(FOCUS_CAMERA) => focus_camera(cameras, transforms, &mut command),
        //mut move_light if move_light.starts_with(MOVE_LIGHT) => move_light(cameras, transforms, &mut camera_movement),
        _ => {}
    };
}

fn move_camera(cameras: ReadStorage<Camera>, mut transforms: WriteStorage<Transform>, camera_movement: &mut String) {
    let camera_movement = camera_movement.split_off(12);
    let args: Vec<&str> = camera_movement.split(' ').collect();
    if args.len() == 3 {
        for (_, transform) in (&cameras, &mut transforms).join() {
            if let Ok(result) = convert_params_to_xyz(args.get(0).unwrap(), args.get(1).unwrap(), args.get(2).unwrap()) {
                let (x, y, z) = result;
                info!("Moving camera to {} {} {}", x, y, z);
                transform.set_translation_xyz(x, y, z);
            }
        }
    }
}

fn focus_camera(cameras: ReadStorage<Camera>, mut transforms: WriteStorage<Transform>, camera_focus_point: &mut String) {
    let camera_focus_point = camera_focus_point.split_off(13);
    let args: Vec<&str> = camera_focus_point.split(' ').collect();
    if args.len() == 3 {
        for (_, transform) in (&cameras, &mut transforms).join() {
            if let Ok(result) = convert_params_to_xyz(args.get(0).unwrap(), args.get(1).unwrap(), args.get(2).unwrap()) {
                let (x, y, z) = result;
                info!("Focusing camera to {} {} {}", x, y, z);
                transform.face_towards(Vector3::new(x, y, z), Vector3::new(0.0, 0.0, 1.0));
            }
        }
    }
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

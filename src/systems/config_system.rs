use amethyst::core::ecs::System;

pub struct ConfigSystem;

impl<'s> System<'s> for ConfigSystem {
    type SystemData = ();
    fn run(&mut self, _data: Self::SystemData) {
        // system needed for configuration
    }
}
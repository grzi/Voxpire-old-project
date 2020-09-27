use amethyst::core::ecs::System;
#[derive(Default)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = ();

    fn run(&mut self, _sd: Self::SystemData) {}
}

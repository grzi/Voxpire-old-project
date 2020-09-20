use amethyst::core::ecs::{System, WriteExpect};
use amethyst::winit::{Window};

pub struct ConfigSystem;

impl<'s> System<'s> for ConfigSystem {
    type SystemData = WriteExpect<'s, Window>;

    fn run(&mut self, _data: Self::SystemData) {
        /* let el = EventsLoop::new();
        match data.get_fullscreen() {
            Some(thing) => data.set_fullscreen(None),
            None => data.set_fullscreen(Some(el.get_primary_monitor()))
        };
        */
    }
}

use amethyst::core::ecs::{DenseVecStorage, Component};

pub struct TimeComponent;

impl Component for TimeComponent {
    type Storage = DenseVecStorage<Self>;
}
use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub v: Vector3<f32>,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

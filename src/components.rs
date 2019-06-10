use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub v: Vector3<f32>,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct Particle {
    pub charges: [usize; 3],
    pub total_charge: isize,
    pub mass: usize,
}

impl Particle {
    pub fn new(charges: [usize; 3]) -> Particle {
        Particle {
            charges: charges,
            total_charge: charges[0] as isize + (charges[2] as isize * -1),
            mass: charges.iter().sum(),
        }
    }
}

impl Component for Particle {
    type Storage = DenseVecStorage<Self>;
}

pub struct LifeTime {
    pub t: f32,
}

impl Component for LifeTime {
    type Storage = DenseVecStorage<Self>;
}

impl LifeTime {
    pub fn new() -> LifeTime {
        LifeTime { t: 0.0 }
    }
}

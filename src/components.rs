use amethyst::core::math::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Velocity {
    pub v: Vector3<f32>,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Particle {
    pub charges: [usize; 3],
    pub total_charge: isize,
    pub mass: usize,
}

impl Particle {
    pub fn new(charges: [usize; 3]) -> Particle {
        let mass = charges.iter().sum();

        if mass == 0 {
            panic!("Cannot create a zero-mass particle!");
        }

        Particle {
            charges: charges,
            total_charge: charges[0] as isize + (charges[2] as isize * -1),
            mass: mass,
        }
    }
}

impl Component for Particle {
    type Storage = DenseVecStorage<Self>;
}

pub struct LifeTime {
    pub t: f32,
    pub decays_after: f32,
}

impl Component for LifeTime {
    type Storage = DenseVecStorage<Self>;
}

impl LifeTime {
    pub fn new(decays_after: f32) -> LifeTime {
        LifeTime {
            t: 0.0,
            decays_after: decays_after,
        }
    }
}

pub struct Trace {
    pub points: Vec<[f32; 2]>,
}

impl Component for Trace {
    type Storage = DenseVecStorage<Self>;
}

impl Trace {
    pub fn new(start_x: f32, start_y: f32) -> Trace {
        Trace {
            points: vec![[start_x, start_y]],
        }
    }
}

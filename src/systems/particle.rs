use crate::components::{Particle, Velocity};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MoveParticleSystem;

impl<'s> System<'s> for MoveParticleSystem {
    type SystemData = (
        ReadStorage<'s, Particle>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (particles, velocities, mut transforms, time): Self::SystemData) {
        for (_particle, velocity, transform) in (&particles, &velocities, &mut transforms).join() {
            let movements = velocity.v * time.delta_seconds();
            transform.translate_xyz(movements[0], movements[1], movements[2]);
        }
    }
}

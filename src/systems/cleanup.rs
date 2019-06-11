use crate::components::{LifeTime, Particle};
use amethyst::ecs::{Entities, Join, System, WriteStorage};

pub struct Cleanup;

impl<'s> System<'s> for Cleanup {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Particle>,
        WriteStorage<'s, LifeTime>,
    );

    fn run(&mut self, (entities, particles, lifetimes): Self::SystemData) {
        for (entity, particle, lifetime) in (&entities, &particles, &lifetimes).join() {
            if particle.mass > 1 {
                continue;
            }

            if particle.total_charge == 0 || lifetime.t > 2.0 {
                entities.delete(entity);
            }
        }
    }
}

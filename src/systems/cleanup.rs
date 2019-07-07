use crate::components::{DeleteFlag, LifeTime, Particle, Trace};
use crate::resources::SVGBuilder;
use amethyst::ecs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct ExpireLifetimes;

impl<'s> System<'s> for ExpireLifetimes {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Particle>,
        WriteStorage<'s, LifeTime>,
        WriteStorage<'s, DeleteFlag>,
    );

    fn run(&mut self, (entities, particles, lifetimes, mut deletes): Self::SystemData) {
        for (entity, particle, lifetime) in (&entities, &particles, &lifetimes).join() {
            if particle.mass > 1 {
                continue;
            }

            if particle.total_charge == 0 || lifetime.t > lifetime.decays_after {
                deletes
                    .insert(entity, DeleteFlag {})
                    .expect("Entity was already marked for deletion!");
            }
        }
    }
}

pub struct Cleanup;

impl<'s> System<'s> for Cleanup {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Trace>,
        ReadStorage<'s, DeleteFlag>,
        Write<'s, SVGBuilder>,
    );

    fn run(&mut self, (entities, traces, deletes, mut svgbuilder): Self::SystemData) {
        // For each entity that has a trace and is about to be deleted, save the trace:
        for (trace, _del) in (&traces, &deletes).join() {
            svgbuilder.paths.push(trace.points.clone());
        }

        // Perform the actual delete
        for (entity, _del) in (&entities, &deletes).join() {
            entities.delete(entity).expect("Failed to delete particle.");
        }
    }
}

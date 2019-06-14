use crate::components::{LifeTime, Particle, Trace};
use crate::resources::SVGBuilder;
use amethyst::ecs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct Cleanup;

impl<'s> System<'s> for Cleanup {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Particle>,
        WriteStorage<'s, LifeTime>,
        ReadStorage<'s, Trace>,
        Write<'s, SVGBuilder>,
    );

    fn run(&mut self, (entities, particles, lifetimes, traces, mut svgbuilder): Self::SystemData) {
        for (entity, particle, lifetime, trace) in
            (&entities, &particles, &lifetimes, &traces).join()
        {
            if particle.mass > 1 {
                continue;
            }

            if particle.total_charge == 0 || lifetime.t > 2.0 {
                // TODO: refactor this common logic:
                svgbuilder.paths.push(trace.points.clone());
                entities.delete(entity).expect("Failed to delete particle.");
            }
        }
    }
}

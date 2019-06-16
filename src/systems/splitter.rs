use amethyst::core::Hidden;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, System, Write, WriteStorage};
use amethyst::renderer::{SpriteRender, Transparent};
use log::info;
use rand::distributions::{Distribution, Exp};
use rand::thread_rng;
use rand::Rng;

use crate::components::{LifeTime, Particle, Trace, Velocity};
use crate::config::MultiParticlesConfig;
use crate::resources::SVGBuilder;

pub struct ParticleSplitter;

impl<'s> System<'s> for ParticleSplitter {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Particle>,
        WriteStorage<'s, LifeTime>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transparent>,
        WriteStorage<'s, Hidden>,
        WriteStorage<'s, Trace>,
        Write<'s, SVGBuilder>,
        Read<'s, MultiParticlesConfig>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut particles,
            mut lifetimes,
            mut transforms,
            mut velocities,
            mut sprites,
            mut transparents,
            mut hidden,
            mut traces,
            mut svgbuilder,
            particles_config,
        ): Self::SystemData,
    ) {
        let mut new_particles = Vec::new();

        for (entity, particle, lifetime, transform, velocity, sprite, trace) in (
            &entities,
            &particles,
            &lifetimes,
            &transforms,
            &velocities,
            &sprites,
            &traces,
        )
            .join()
        {
            if lifetime.t < lifetime.decays_after || particle.mass == 1 {
                continue;
            }

            new_particles
                .append(&mut self.split_particle(&particle, &transform, &velocity, &sprite));

            // TODO: refactor this common logic:
            svgbuilder.paths.push(trace.points.clone());
            entities.delete(entity).expect("Failed to delete particle.");
        }

        let decay_rate = particles_config.decay_rate;

        let mut rng = thread_rng();
        let decay_distribution = Exp::new(decay_rate as f64);

        for (particle, transform, velocity, sprite) in new_particles {
            let total_charge = particle.total_charge;
            let location = transform.translation();

            let mut entity = entities
                .build_entity()
                .with(particle, &mut particles)
                .with(
                    LifeTime::new(decay_distribution.sample(&mut rng) as f32),
                    &mut lifetimes,
                )
                .with(
                    Trace::new(location[0].as_f32(), location[1].as_f32()),
                    &mut traces,
                )
                .with(transform, &mut transforms)
                .with(velocity, &mut velocities)
                .with(sprite.clone(), &mut sprites)
                .with(Transparent, &mut transparents);

            if total_charge == 0 {
                // Particles without charge don't show
                entity = entity.with(Hidden, &mut hidden);
            }

            entity.build();
        }
    }
}

impl ParticleSplitter {
    fn split_particle(
        &self,
        particle: &Particle,
        transform: &Transform,
        velocity: &Velocity,
        sprite: &SpriteRender,
    ) -> Vec<(Particle, Transform, Velocity, SpriteRender)> {
        let mut random = rand::thread_rng();

        // let mut n_new_parts = Poisson::new(2.0).sample(&mut random) as u8;
        // n_new_parts = cmp::max(n_new_parts, 2);
        // n_new_parts = cmp::min(n_new_parts, particle.mass as u8);
        // info!("Splitting particle {:?} in {}", particle, n_new_parts);

        let mut charges_left = particle.charges;

        let mut results = Vec::new();

        while charges_left[0] > 0 || charges_left[1] > 0 || charges_left[2] > 0 {
            let pos = if charges_left[0] > 0 {
                random.gen_range(0, charges_left[0] + 1)
            } else {
                0
            };
            let neutral = if charges_left[1] > 0 {
                random.gen_range(0, charges_left[1] + 1)
            } else {
                0
            };
            let neg = if charges_left[2] > 0 {
                random.gen_range(0, charges_left[2] + 1)
            } else {
                0
            };

            // Disregard zero-mass particles
            if pos == 0 && neutral == 0 && neg == 0 {
                continue;
            }

            info!("New particle with charge {},{},{}", pos, neutral, neg);

            charges_left[0] -= pos;
            charges_left[1] -= neutral;
            charges_left[2] -= neg;

            results.push((
                Particle::new([pos, neutral, neg]),
                transform.clone(),
                velocity.clone(),
                sprite.clone(),
            ));
        }

        results
    }
}

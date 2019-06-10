use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, System, WriteStorage};
use amethyst::renderer::{Hidden, SpriteRender, Transparent};
use log::info;
use rand::Rng;

use crate::components::{LifeTime, Particle, Velocity};

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
        ): Self::SystemData,
    ) {
        let mut new_particles = Vec::new();

        for (entity, particle, lifetime, transform, velocity, sprite) in (
            &entities,
            &particles,
            &lifetimes,
            &transforms,
            &velocities,
            &sprites,
        )
            .join()
        {
            // TODO: Sample from exponential decay distribution?
            if lifetime.t < 2.0 || particle.mass == 1 {
                continue;
            }

            new_particles
                .append(&mut self.split_particle(&particle, &transform, &velocity, &sprite));
            entities.delete(entity);
        }

        for (particle, transform, velocity, sprite) in new_particles {
            let total_charge = particle.total_charge;
            let mut entity = entities
                .build_entity()
                .with(particle, &mut particles)
                .with(LifeTime::new(), &mut lifetimes)
                .with(transform, &mut transforms)
                .with(velocity, &mut velocities)
                .with(sprite.clone(), &mut sprites)
                .with(Transparent, &mut transparents);

            // Particles without charge don't show
            if total_charge == 0 {
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

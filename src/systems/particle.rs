use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::renderer::{SpriteRender, Transparent};
use log::{info, trace, warn};
use rand::distributions::{Distribution, Poisson};
use rand::Rng;
use std::cmp;

use crate::components::{LifeTime, Particle, Velocity};
use crate::resources::MagneticField;

pub struct MoveByVelocity;

impl<'s> System<'s> for MoveByVelocity {
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

pub struct MagneticForce;

impl<'s> System<'s> for MagneticForce {
    type SystemData = (
        ReadStorage<'s, Particle>,
        WriteStorage<'s, Velocity>,
        ReadExpect<'s, MagneticField>,
        Read<'s, Time>,
    );

    fn run(&mut self, (particles, mut velocities, magnetic_field, time): Self::SystemData) {
        for (particle, velocity) in (&particles, &mut velocities).join() {
            // Magnetic component of Lorentz force:
            let force = particle.total_charge as f32 * (velocity.v.cross(&magnetic_field.field));

            // (F = m.a), so (a = F/m)
            let acceleration = force / particle.mass as f32;

            velocity.v += acceleration;
        }
    }
}

pub struct LifeTimeCounter;

impl<'s> System<'s> for LifeTimeCounter {
    type SystemData = (WriteStorage<'s, LifeTime>, Read<'s, Time>);

    fn run(&mut self, (mut lifetimes, time): Self::SystemData) {
        for lifetime in (&mut lifetimes).join() {
            lifetime.t += time.delta_seconds();
        }
    }
}

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
            if lifetime.t < 2.0 {
                continue;
            }

            new_particles
                .append(&mut self.split_particle(&particle, &transform, &velocity, &sprite));
            entities.delete(entity);
        }

        for (particle, transform, velocity, sprite) in new_particles {
            entities
                .build_entity()
                .with(particle, &mut particles)
                .with(LifeTime::new(), &mut lifetimes)
                .with(transform, &mut transforms)
                .with(velocity, &mut velocities)
                .with(sprite.clone(), &mut sprites)
                .with(Transparent, &mut transparents)
                .build();
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

        let mut n_new_parts = Poisson::new(2.0).sample(&mut random) as u8;
        n_new_parts = cmp::max(n_new_parts, 2);
        n_new_parts = cmp::min(n_new_parts, particle.mass as u8);
        info!("Splitting particle {:?} in {}", particle, n_new_parts);

        let mut charges_left = particle.charges;

        let mut results = Vec::new();

        for _ in 1..n_new_parts {
            let pos = if charges_left[0] > 0 {
                random.gen_range(0, charges_left[0])
            } else {
                0
            };
            let neutral = if charges_left[1] > 0 {
                random.gen_range(0, charges_left[0])
            } else {
                0
            };
            let neg = if charges_left[2] > 0 {
                random.gen_range(0, charges_left[0])
            } else {
                0
            };
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

        info!(
            "New particle with charge {},{},{}",
            charges_left[0], charges_left[1], charges_left[2]
        );
        // Remaining charges go to final particle
        results.push((
            Particle::new(charges_left),
            transform.clone(),
            velocity.clone(),
            sprite.clone(),
        ));

        results
    }
}

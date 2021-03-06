use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::{Particle, Velocity};
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
            transform.prepend_translation(movements);
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

            velocity.v += acceleration * time.delta_seconds();
        }
    }
}

pub struct Exhaustion;

impl<'s> System<'s> for Exhaustion {
    type SystemData = (WriteStorage<'s, Velocity>, Read<'s, Time>);

    fn run(&mut self, (mut velocities, time): Self::SystemData) {
        // It is known that a charged particle’s range d,
        // which is the distance it travelled beforecoming to rest,
        // is approximately proportional to the fourth power of its initial momentum,
        // (i.e.,d∝p4)

        // Normal drag is proportional to velocity or velocity squared,
        // So it seems velocity is always involved...

        for velocity in (&mut velocities).join() {
            // Simulate some simple friction
            velocity.v *= 1.0 - (0.3 * time.delta_seconds());
        }
    }
}

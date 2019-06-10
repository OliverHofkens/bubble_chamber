use crate::components::LifeTime;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, System, WriteStorage};

pub struct LifeTimeCounter;

impl<'s> System<'s> for LifeTimeCounter {
    type SystemData = (WriteStorage<'s, LifeTime>, Read<'s, Time>);

    fn run(&mut self, (mut lifetimes, time): Self::SystemData) {
        for lifetime in (&mut lifetimes).join() {
            lifetime.t += time.delta_seconds();
        }
    }
}

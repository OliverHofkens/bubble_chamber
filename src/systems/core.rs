use crate::components::LifeTime;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::utils::fps_counter::FPSCounter;
use log::info;

pub struct LifeTimeCounter;

impl<'s> System<'s> for LifeTimeCounter {
    type SystemData = (WriteStorage<'s, LifeTime>, Read<'s, Time>);

    fn run(&mut self, (mut lifetimes, time): Self::SystemData) {
        for lifetime in (&mut lifetimes).join() {
            lifetime.t += time.delta_seconds();
        }
    }
}

pub struct LogFPS;

impl<'s> System<'s> for LogFPS {
    type SystemData = (Read<'s, FPSCounter>);

    fn run(&mut self, (fps_counter): Self::SystemData) {
        info!("AVG FPS: {}", fps_counter.sampled_fps());
    }
}

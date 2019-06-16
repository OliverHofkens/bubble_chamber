use crate::components::Trace;
use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

pub struct TraceBuilder;

impl<'s> System<'s> for TraceBuilder {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Trace>,
        ReadStorage<'s, Hidden>,
    );

    fn run(&mut self, (transforms, mut traces, hiddens): Self::SystemData) {
        for (transform, trace, _) in (&transforms, &mut traces, !&hiddens).join() {
            let trans = transform.translation();
            trace.points.push([trans[0].as_f32(), trans[1].as_f32()]);
        }
    }
}

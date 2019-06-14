use crate::components::Trace;
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

pub struct TraceBuilder;

impl<'s> System<'s> for TraceBuilder {
    type SystemData = (ReadStorage<'s, Transform>, WriteStorage<'s, Trace>);

    fn run(&mut self, (transforms, mut traces): Self::SystemData) {
        for (transform, trace) in (&transforms, &mut traces).join() {
            let trans = transform.translation();
            trace.points.push([trans[0], trans[1]]);
        }
    }
}

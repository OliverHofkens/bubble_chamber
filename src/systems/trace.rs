use crate::components::Trace;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};
use amethyst::renderer::{SpriteRender, Transparent};

pub struct TraceBuilder;

impl<'s> System<'s> for TraceBuilder {
    type SystemData = (ReadStorage<'s, Transform>, WriteStorage<'s, Trace>);

    fn run(&mut self, (transforms, mut traces): Self::SystemData) {
        for (transform, trace) in (&transforms, &mut traces).join() {
            let trans = transform.translation();
            trace.points.push([trans[0].as_f32(), trans[1].as_f32()]);
        }
    }
}

pub struct PersistentTrail;

impl<'s> System<'s> for PersistentTrail {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Trace>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transparent>,
    );

    fn run(
        &mut self,
        (entities, mut transforms, traces, mut sprites, mut transparents): Self::SystemData,
    ) {
        let mut to_create = Vec::new();

        for (transform, _trace, sprite) in (&transforms, &traces, &sprites).join() {
            to_create.push((transform.clone(), sprite.clone()));
        }

        for (loc, sprite) in to_create {
            entities
                .build_entity()
                .with(loc, &mut transforms)
                .with(Transparent, &mut transparents)
                .with(sprite.clone(), &mut sprites)
                .build();
        }
    }
}

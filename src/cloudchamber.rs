use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Vector3;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Hidden, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata, Transparent,
};

use crate::components::{LifeTime, Particle, Velocity};
use crate::resources::MagneticField;

pub const ARENA_HEIGHT: f32 = 1080.0;
pub const ARENA_WIDTH: f32 = 1920.0;

pub struct CloudChamber;

impl SimpleState for CloudChamber {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_particles(world, sprite_sheet_handle);
        initialise_camera(world);

        world.add_resource(MagneticField {
            field: Vector3::new(0.0, 0.0, 2.0),
        });
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_particles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();

    // Correctly position the particles
    let y = ARENA_HEIGHT / 2.0;
    transform.set_xyz(0.0, y, 0.0);

    // Assign the sprite for the particles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // particle is the first and only sprite in the sprite_sheet
    };

    let particle = Particle::new([10, 10, 10]);
    let total_charge = particle.total_charge;
    let mut entity = world
        .create_entity()
        .with(particle)
        .with(LifeTime::new())
        .with(transform)
        .with(Velocity {
            v: Vector3::new(500.0, 0.0, 0.0),
        })
        .with(sprite_render.clone())
        .with(Transparent);

    // Neutral particles do not leave tracks
    if total_charge == 0 {
        entity = entity.with(Hidden);
    }

    entity.build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

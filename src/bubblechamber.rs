use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Vector3;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Hidden, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata, Transparent,
};

use crate::components::{LifeTime, Particle, Velocity};
use crate::config::{ChamberConfig, MagneticFieldConfig, MultiParticlesConfig};
use crate::resources::MagneticField;

pub struct BubbleChamber;

impl SimpleState for BubbleChamber {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_particles(world, sprite_sheet_handle);
        initialise_magnetic_field(world);
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    let (chamber_width, chamber_height) = {
        let config = &world.read_resource::<ChamberConfig>();
        (config.width, config.height)
    };

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            chamber_width,
            0.0,
            chamber_height,
        )))
        .with(transform)
        .build();
}

fn initialise_particles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let particle_configs: Vec<([usize; 3], Vector3<f32>, Vector3<f32>)> = {
        let config = &world.read_resource::<MultiParticlesConfig>();

        config
            .at_start
            .iter()
            .map(|conf| (conf.charges, conf.location, conf.velocity))
            .collect()
    };

    for (charges, location, velocity) in particle_configs {
        let particle = Particle::new(charges);
        let mut transform = Transform::default();
        transform.set_xyz(location[0], location[1], location[2]);
        let velocity = Velocity { v: velocity };

        // Assign the sprite for the particles
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0, // particle is the first and only sprite in the sprite_sheet
        };
        let total_charge = particle.total_charge;

        let mut entity = world
            .create_entity()
            .with(particle)
            .with(LifeTime::new())
            .with(transform)
            .with(velocity)
            .with(sprite_render.clone())
            .with(Transparent);

        // Neutral particles do not leave tracks
        if total_charge == 0 {
            entity = entity.with(Hidden);
        }

        entity.build();
    }
}

fn initialise_magnetic_field(world: &mut World) {
    let field = {
        let config = &world.read_resource::<MagneticFieldConfig>();
        config.field
    };

    world.add_resource(MagneticField { field: field });
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

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Vector3;
use amethyst::core::transform::Transform;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::VirtualKeyCode;
use amethyst::renderer::{
    Camera, Hidden, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata, Transparent,
};
use rand::distributions::{Distribution, Exp};
use rand::thread_rng;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle};
use svg::Document;

use crate::components::{LifeTime, Particle, Trace, Velocity};
use crate::config::{ChamberConfig, MagneticFieldConfig, MultiParticlesConfig};
use crate::resources::{MagneticField, SVGBuilder};

pub struct BubbleChamber;

impl SimpleState for BubbleChamber {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_particles(world, sprite_sheet_handle);
        initialise_magnetic_field(world);
        initialise_camera(world);
        initialise_svg(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // On stop, we construct the SVG:
        output_svg(data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Pause the game by going to the `PausedState`.
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this `State`.
        Trans::None
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
    let (decay_rate, particle_configs): (f32, Vec<([usize; 3], Vector3<f32>, Vector3<f32>)>) = {
        let config = &world.read_resource::<MultiParticlesConfig>();

        (
            config.decay_rate,
            config
                .at_start
                .iter()
                .map(|conf| (conf.charges, conf.location, conf.velocity))
                .collect(),
        )
    };
    let mut rng = thread_rng();
    let decay_distribution = Exp::new(decay_rate as f64);

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
            .with(LifeTime::new(decay_distribution.sample(&mut rng) as f32))
            .with(transform)
            .with(velocity)
            .with(sprite_render.clone())
            .with(Trace::new(location[0], location[1]))
            .with(Transparent);

        if total_charge == 0 {
            // Neutral particles do not leave tracks
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

fn initialise_svg(world: &mut World) {
    world.add_resource(SVGBuilder { paths: Vec::new() });
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

pub fn output_svg(world: &mut World) {
    let svg_builder = world.read_resource::<SVGBuilder>();

    let (viewbox_width, viewbox_height) = {
        let config = world.read_resource::<ChamberConfig>();
        (config.width, config.height)
    };
    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));

    // Add 100% rect in black as background
    document = document.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "black"),
    );

    for path in &svg_builder.paths {
        // Starting point
        let mut data = Data::new().move_to((path[0][0], path[0][1]));

        // Make the curve
        let all_points: Vec<f32> = path[1..].iter().flatten().cloned().collect();
        data = data.cubic_curve_to(all_points);

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "white")
            .set("stroke-width", 3)
            .set("d", data.to_owned());

        document = document.add(path);
    }

    svg::save("particles.svg", &document).unwrap();
}

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
};
use amethyst::utils::application_root_dir;
// use amethyst::utils::fps_counter::FPSCounterBundle;

mod bubblechamber;
mod components;
mod config;
mod resources;
mod systems;

use crate::bubblechamber::BubbleChamber;
use crate::config::SimulationConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let path = format!("{}/resources/sim_config.ron", application_root_dir());
    let simulation_config = SimulationConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            // .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]),
        )?
        .with_bundle(TransformBundle::new())?
        // .with_bundle(FPSCounterBundle::default())?
        // .with(systems::LogFPS, "log_fps", &[])
        .with(systems::LifeTimeCounter, "lifetime_counter", &[])
        .with(systems::MoveByVelocity, "move_by_velocity", &[])
        .with(systems::MagneticForce, "magnetic_force", &[])
        .with(systems::Exhaustion, "exhaustion", &[])
        .with(systems::ParticleSplitter, "particle_splitter", &[])
        .with(systems::Cleanup, "cleanup", &[]);

    let mut game = Application::build("./", BubbleChamber)
        .expect("Failed to initialize")
        .with_resource(simulation_config.chamber)
        .with_resource(simulation_config.magnetic_field)
        .with_resource(simulation_config.particles)
        .build(game_data)
        .expect("Failed to build game");

    game.run();

    Ok(())
}

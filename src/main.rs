use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};

mod bubblechamber;
mod components;
mod config;
mod resources;
mod systems;

use crate::bubblechamber::BubbleChamber;
use crate::config::SimulationConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("config");
    let display_config_path = resources_dir.join("display_config.ron");

    let assets_dir = app_root.join("assets");

    let simulation_config_path = resources_dir.join("sim_config.ron");
    let simulation_config = SimulationConfig::load(&simulation_config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        // .with_bundle(FpsCounterBundle::default())?
        // .with(systems::LogFps, "log_fps", &[])
        .with(systems::LifeTimeCounter, "lifetime_counter", &[])
        .with(systems::MagneticForce, "magnetic_force", &[])
        .with(systems::Exhaustion, "exhaustion", &[])
        .with(
            systems::MoveByVelocity,
            "move_by_velocity",
            &["magnetic_force", "exhaustion"],
        )
        .with(
            systems::ExpireLifetimes,
            "expire_lifetimes",
            &["move_by_velocity"],
        )
        .with(
            systems::ParticleSplitter,
            "particle_splitter",
            &["move_by_velocity"],
        )
        .with(
            systems::TraceBuilder,
            "svg_path_builder",
            &["particle_splitter"],
        )
        .with(
            systems::PersistentTrail,
            "persistent_trail",
            &["particle_splitter"],
        )
        .with(
            systems::Cleanup,
            "cleanup",
            &["particle_splitter", "expire_lifetimes"],
        );

    let mut game = Application::build(assets_dir, BubbleChamber)
        .expect("Failed to initialize")
        .with_resource(simulation_config.chamber)
        .with_resource(simulation_config.magnetic_field)
        .with_resource(simulation_config.particles)
        .build(game_data)
        .expect("Failed to build game");

    game.run();

    Ok(())
}

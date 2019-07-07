use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        sprite_visibility::SpriteVisibilitySortingSystem, types::DefaultBackend, RenderingSystem,
        SpriteSheet,
    },
    utils::{application_root_dir, fps_counter::FPSCounterBundle},
    window::WindowBundle,
};

mod bubblechamber;
mod components;
mod config;
mod render_graph;
mod resources;
mod systems;

use crate::bubblechamber::BubbleChamber;
use crate::config::SimulationConfig;
use crate::render_graph::RenderGraph;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");

    let assets_dir = app_root.join("assets");

    let simulation_config_path = resources_dir.join("sim_config.ron");
    let simulation_config = SimulationConfig::load(&simulation_config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ))
        .with_bundle(TransformBundle::new())?
        // .with_bundle(FPSCounterBundle::default())?
        // .with(systems::LogFPS, "log_fps", &[])
        .with(systems::LifeTimeCounter, "lifetime_counter", &[])
        .with(systems::MagneticForce, "magnetic_force", &[])
        .with(systems::Exhaustion, "exhaustion", &[])
        .with(
            systems::MoveByVelocity,
            "move_by_velocity",
            &["magnetic_force", "exhaustion"],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
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
            &["move_by_velocity"],
        )
        .with(systems::PersistentTrail, "persistent_trail", &["move_by_velocity"])
        .with(
            systems::ExpireLifetimes,
            "expire_lifetimes",
            &["move_by_velocity"],
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

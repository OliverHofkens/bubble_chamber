extern crate amethyst;
use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
};
use amethyst::utils::application_root_dir;
mod cloudchamber;
mod components;
mod systems;

use crate::cloudchamber::CloudChamber;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]),
        )?
        .with_bundle(TransformBundle::new())?
        .with(systems::MoveParticleSystem, "move_particle_system", &[]);

    let mut game = Application::new("./", CloudChamber, game_data)?;

    game.run();

    Ok(())
}

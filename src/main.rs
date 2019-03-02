#![warn(clippy::all)]
extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
    LoggerConfig,
    core::transform::TransformBundle
};
use log::LevelFilter;

pub mod internals;
pub mod components;
pub mod systems;
pub mod bundle;
pub mod states;

use crate::bundle::*;
use crate::states::simulation::Simulation;

fn main() -> amethyst::Result<()> {
    let mut logger_config = LoggerConfig::default();
    logger_config.level_filter = LevelFilter::Error;
    amethyst::start_logger(logger_config);

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let game_data =
        GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(AntSimBundle{})?
        ;
    let mut game = Application::new("./", Simulation::new(), game_data)?;

    game.run();

    Ok(())
}

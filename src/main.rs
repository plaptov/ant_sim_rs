#![warn(clippy::all)]
extern crate ggez;
extern crate gfx;

use std::path;
use ggez::conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

pub mod internals;
pub mod components;
pub mod states;

use crate::states::simulation::Simulation;

fn main() -> GameResult<()> {
    color_backtrace::install();
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ContextBuilder::new("antsim", "ggez")
        .window_setup(conf::WindowSetup::default().title("Ants simulation").resizable(false))
        .window_mode(conf::WindowMode::default().dimensions(800, 400))
        .add_resource_path(resource_dir);

    let ctx = &mut cb.build()?;

    let game = &mut Simulation::new(ctx)?;
    event::run(ctx, game)
}

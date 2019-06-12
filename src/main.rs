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
pub mod drawing;

use crate::states::simulation::Simulation;

fn main() -> GameResult<()> {
    color_backtrace::install();
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ContextBuilder::new("antsim", "ggez")
        .window_setup(conf::WindowSetup::default().title("Ants simulation"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 400.0).resizable(false))
        .add_resource_path(resource_dir);

    let (ctx, events) = &mut cb.build()?;

    let game = &mut Simulation::new(ctx)?;
    event::run(ctx, events, game)
}

use amethyst::ecs::{Component, DenseVecStorage};
use crate::internals::coordinate::Coordinate;

pub struct Food {
    pub pos: Coordinate,
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}

impl Food {
    pub fn new(pos: Coordinate) -> Food {
        Food {
            pos
        }
    }
}
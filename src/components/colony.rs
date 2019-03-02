use amethyst::ecs::{Component, DenseVecStorage};
use specs::world::Index;
use crate::internals::coordinate::Coordinate;
use crate::components::ant::Ant;

pub struct Colony {
    pub home: Coordinate,
    pub ants_count: u32,
    pub max_ants: u32,
}

impl Component for Colony {
    type Storage = DenseVecStorage<Self>;
}

impl Colony {
    
    pub fn new(home: Coordinate, max_ants: u32) -> Colony {
        Colony {
            home,
            ants_count: 0,
            max_ants,
        }
    }

    pub fn make_ant(&mut self, colony_id: Index) -> Ant {
        self.ants_count += 1;
        Ant::new(self, colony_id)
    }

    pub fn dead(&mut self, _ant: &Ant) {
        self.ants_count -= 1;
    }

}
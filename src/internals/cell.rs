use crate::internals::coordinate::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Cell {
    pub position: Coordinate,
    pub pheromones: i32,
    pub food: i32,
    pub is_obstacle: bool,
}

impl Cell {
    pub fn new(position: Coordinate) -> Cell {
        Cell {
            position,
            pheromones: 0,
            food: 0,
            is_obstacle: false
        }
    }

    pub fn new_ex(position: Coordinate, is_obstacle: bool) -> Cell {
        Cell {
            position,
            pheromones: 0,
            food: 0,
            is_obstacle
        }
    }

    #[inline(always)]
    pub fn get_attraction(&self) -> i32 {
        10
        + self.pheromones
        + self.food
    }

    #[inline(always)]
    pub fn distance_to(&self, other: &Cell) -> f32 {
        self.position.distance_to(other.position)
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        if self.pheromones > 0 {
            self.pheromones -= 1;
        }
    }
}
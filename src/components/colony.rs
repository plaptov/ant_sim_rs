use crate::internals::coordinate::Coordinate;
use crate::internals::field::Field;
use crate::components::ant::*;

pub struct Colony {
    pub home: Coordinate,
    pub max_ants: usize,
    ants: Vec<Ant>,
}

impl Colony {
    
    pub fn new(home: Coordinate, max_ants: usize) -> Colony {
        Colony {
            home,
            max_ants,
            ants: Vec::with_capacity(max_ants),
        }.inhabit()
    }

    fn inhabit(mut self) -> Self {
        for _ in 0..self.max_ants {
            self.ants.push(Ant::new(self.home));
        }
        self
    }

    pub fn check_cells(&mut self, field: &mut Field) {
        for ant in &mut self.ants {
            ant.check_current_cell(field);
        }
    }

    pub fn move_ants(&mut self, field: &mut Field) {
        for ant in self.ants.iter_mut() {
            match ant.make_move(field) {
                AntMoveResult::Ok => {},
                AntMoveResult::Died => {
                    *ant = Ant::new(self.home);
                }
            }
        }
    }

}
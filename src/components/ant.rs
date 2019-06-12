use rand::Rng;

use crate::internals::cell::Cell;
use crate::internals::coordinate::Coordinate;
use crate::internals::field::Field;

pub struct Ant {
    home: Coordinate,
    pub current_cell: Coordinate,
    is_returning: bool,
    is_good_returning: bool,
    distance_to_food: i32,
    current_path: Vec<Coordinate>,
}

#[derive(PartialEq, Eq)]
pub enum AntMoveResult {
    Ok,
    Died,
}

impl Ant {
    pub fn new(home: Coordinate) -> Ant {
        Ant {
            home,
            current_cell: home,
            is_returning: false,
            is_good_returning: false,
            distance_to_food: 0,
            current_path: vec![ home ],
        }
    }

    fn step_to(&mut self, pos: Coordinate) {
        if self.is_returning 
            && self.current_path.len() > 1 
            && self.current_path.last() == Some(&pos) 
        {
            self.current_path.pop();
        }
        if !self.is_returning {
            self.current_path.push(pos);
        }
        self.current_cell = pos;
    }

    fn can_move_to(&self, cell: &Cell) -> bool {
        let pos = cell.position;
        !cell.is_obstacle 
        && pos != self.current_cell
        && (self.is_returning || !self.current_path.contains(&pos))
        && (!self.is_returning || self.current_path.last() == Some(&pos))
    }

    fn die(&mut self, field: &mut Field) {
        field.get_mut_by_pos(self.current_cell).ants -= 1;
    }

    pub fn make_move(&mut self, field: &mut Field) -> AntMoveResult {
        if self.current_cell == self.home {
            self.is_returning = false;
            self.is_good_returning = false;
        }
        if self.is_returning {
            if let Some(last_cell) = self.current_path.last() {
                if *last_cell == self.current_cell {
                    self.current_path.pop();
                }
            }
        }

        let steps: Vec<&Cell> = 
            field
            .steps_from_pos(self.current_cell)
            .iter()
            .filter(|x| self.can_move_to(x))
            .cloned()
            .collect();

        if steps.is_empty() {
            if !self.is_returning {
                self.is_returning = true;
                return AntMoveResult::Ok;
            }
            else {
                self.die(field);
                return AntMoveResult::Died;
            }
        }

        let mut next_pos = None;

        let sum_attraction: i32 = steps.iter().map(|x| x.get_attraction()).sum();
        let mut val = rand::thread_rng().gen_range(0, sum_attraction);
        for step in steps {
            let att = step.get_attraction();
            if val <= att {
                next_pos = Some(step.position);
                break;
            }
            val -= att;
        }

        if let Some(pos) = next_pos {
            field.get_mut_by_pos(self.current_cell).ants -= 1;
            self.step_to(pos);
            field.get_mut_by_pos(self.current_cell).ants += 1;
        }

        AntMoveResult::Ok
    }

    fn pheromone_count_to_put(&self) -> i32 {
        self.distance_to_food * 10
    }

    pub fn check_current_cell(&mut self, field: &mut Field) {
        let cell = field.get_mut_by_pos(self.current_cell);
        if cell.food > 0 {
            cell.food -= 1;
            self.distance_to_food = self.current_cell.distance_to(self.home) as i32;
            self.is_returning = true;
            self.is_good_returning = true;
        }
        if self.is_good_returning {
            cell.pheromones += self.pheromone_count_to_put();
        }
    }
}
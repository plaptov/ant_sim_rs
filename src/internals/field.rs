use crate::internals::coordinate::*;
use crate::internals::cell::*;
use std::ops::IndexMut;
use rand::Rng;

/*

Field store cells in one-dimensional array in folowing order:
width = 4, height = 3
ind [x,y]
0   [0,0]
1   [0,1]
2   [0,2]
3   [1,0] 1*height + 0
4   [1,1]
5   [1,2] 1*height + 2
6   [2,0]
7   [2,1]
8   [2,2]
9   [3,0]
10  [3,1] 3*height + 1
11  [3,2]

*/

pub struct Field {
    pub width: i32,
    pub height: i32,
    cells: Vec<Cell>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Field {
        let mut cells = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let is_obstacle =
                    //(x & 1 == 1) &&
                    //(y & 1 == 1) &&
                    rand::thread_rng().gen_range(0, 100) < 20
                    ;
                cells.push(
                    Cell::new_ex(Coordinate::new(x, y), is_obstacle)
                );
            }
        }
        Field {
            width,
            height,
            cells,
        }
    }

    pub const fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    #[inline(always)]
    const fn get_index_in_vec(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }

    #[inline(always)]
    pub fn get(&self, x: i32, y: i32) -> &Cell {
        &self.cells[self.get_index_in_vec(x, y)]
    }
    
    #[inline(always)]
    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut Cell {
        self.cells.index_mut(self.get_index_in_vec(x, y))
    }
    
    #[inline(always)]
    pub fn get_as_mut(&mut self, cell: &Cell) -> &mut Cell {
        self.get_mut(cell.position.x, cell.position.y)
    }

    #[inline(always)]
    pub fn get_by_pos(&self, pos: Coordinate) -> &Cell {
        self.get(pos.x, pos.y)
    }

    #[inline(always)]
    pub fn get_mut_by_pos(&mut self, pos: Coordinate) -> &mut Cell {
        self.get_mut(pos.x, pos.y)
    }

    pub fn tick(&mut self) {
        for cell in &mut self.cells {
            cell.tick();
        }
    }

    pub fn steps_from_pos<'a>(&'a self, pos: Coordinate) -> Vec<&'a Cell> {
        self.steps_from(self.get_by_pos(pos))
    }

    pub fn steps_from<'a>(&'a self, cell: &Cell) -> Vec<&'a Cell> {
        let x = cell.position.x;
        let y = cell.position.y;
        let mut steps = Vec::new();
        let mut push_with_check = | cell: &'a Cell | {
            if !cell.is_obstacle {
                steps.push(cell);
            }
        };
        if x > 0 && y > 0 {
            push_with_check(self.get(x - 1, y - 1));
        }
        if y > 0 {
            push_with_check(self.get(x, y - 1));
        }
        if x < (self.width - 1) && y > 0 {
            push_with_check(self.get(x + 1, y - 1));
        }
        if x < (self.width - 1) {
            push_with_check(self.get(x + 1, y));
        }
        if x < (self.width - 1) && y < (self.height - 1) {
            push_with_check(self.get(x + 1, y + 1));
        }
        if y < (self.height - 1) {
            push_with_check(self.get(x, y + 1));
        }
        if x > 0 && y < (self.height - 1) {
            push_with_check(self.get(x - 1, y + 1));
        }
        if x > 0 {
            push_with_check(self.get(x - 1, y));
        }

        steps
    }

    pub fn place_food_by_pos(&mut self, pos: Coordinate) {
        let mut cell = self.get_mut_by_pos(pos);
        cell.food += 10000;
    }
}
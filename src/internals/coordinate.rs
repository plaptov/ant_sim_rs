use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }

    pub fn distance_to(self, other: Coordinate) -> f32 {
        let xdif = self.x - other.x;
        let ydif = self.y - other.y;
        let f = (xdif * xdif + ydif * ydif) as f32;
        f.sqrt()
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
use crate::internals::coordinate::Coordinate;

pub struct Food {
    pub pos: Coordinate,
}

impl Food {
    pub fn new(pos: Coordinate) -> Food {
        Food {
            pos
        }
    }
}
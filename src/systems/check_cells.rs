use amethyst::ecs::{System, WriteStorage, WriteExpect};
use amethyst::ecs::Join;
use crate::components::ant::Ant;
use crate::internals::field::Field;

pub struct CheckCells {

}

impl<'a> System<'a> for CheckCells {
    type SystemData = (
        WriteStorage<'a, Ant>,
        WriteExpect<'a, Field>,
    );

    fn run(&mut self, (mut ants, mut field): Self::SystemData) {
        for ant in (&mut ants).join() {
            ant.check_current_cell(&mut field);
        }
    }
}
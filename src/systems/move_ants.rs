use amethyst::ecs::{System, WriteStorage, ReadExpect, Entities};
use amethyst::ecs::Join;
use amethyst::core::transform::Transform;
use ignore_result::Ignore;
use crate::components::ant::{Ant, AntMoveResult};
use crate::components::colony::Colony;
use crate::internals::field::Field;

pub struct MoveAnts {

}

impl<'a> System<'a> for MoveAnts {
    type SystemData = (
        WriteStorage<'a, Colony>,
        WriteStorage<'a, Ant>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Field>,
        Entities<'a>,
    );

    fn run(&mut self, (mut colonies, mut ants, mut transforms, field, entities): Self::SystemData) {
        for (e, ant, transform) in (&*entities, &mut ants, &mut transforms).join() {
            match ant.make_move(&field) {
                AntMoveResult::Died => { 
                    entities.delete(e).ignore();
                    let colony_entity = entities.entity(ant.colony_id);
                    let colony = colonies.get_mut(colony_entity).unwrap();
                    colony.dead(ant);
                },
                AntMoveResult::Ok => { 
                    transform.set_xyz(ant.current_cell.x as f32, ant.current_cell.y as f32, 0.0);
                },
            }
        }
    }
}
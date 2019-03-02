use amethyst::ecs::{System, WriteStorage, Entities};
use amethyst::core::transform::Transform;
use amethyst::ecs::Join;
use amethyst::renderer::SpriteRender;

use crate::components::ant::Ant;
use crate::components::colony::Colony;

pub struct SpawnAnts {
    ant_render: Option<SpriteRender>,
}

impl Default for SpawnAnts {
    fn default() -> SpawnAnts {
        SpawnAnts {
            ant_render: None,
        }
    }
}

impl SpawnAnts {
    pub fn new(ant_render: SpriteRender) -> SpawnAnts {
        SpawnAnts {
            ant_render: Some(ant_render)
        }
    }
}

impl<'a> System<'a> for SpawnAnts {
    type SystemData = (
        WriteStorage<'a, Colony>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Ant>,
        Entities<'a>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(&mut self, (mut colonies, mut transforms, mut ants, entities, mut renders): Self::SystemData) {
        if let Some(render) = &self.ant_render {
            for (colony, e) in (&mut colonies, &entities).join() {
                if colony.ants_count < colony.max_ants {
                    let ant = colony.make_ant(e.id());
                    let mut transform = Transform::default();
                    transform.set_xyz(ant.current_cell.x as f32, ant.current_cell.y as f32, 0.0);

                    entities.build_entity()
                        .with(ant, &mut ants)
                        .with(render.clone(), &mut renders)
                        .with(transform, &mut transforms)
                        .build();
                }
            }
        }
    }

}
extern crate amethyst;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::ecs::prelude::*;

use crate::internals::field::Field;
use crate::internals::coordinate::Coordinate;
use crate::systems::{
    spawn_ants::SpawnAnts,
    check_cells::CheckCells,
    move_ants::MoveAnts,
};
use crate::components::{
    colony::Colony,
    ant::Ant,
    food::Food,
};
use rand::Rng;

pub const FIELD_WIDTH: i32 = 800;
pub const FIELD_HEIGHT: i32 = 400;
pub const COLONY_COUNT: i32 = 5;
pub const FOOD_COUNT: u32 = 20;

#[derive(Default)]
pub struct Simulation<'d, 'e,> {
    main_dispatcher: Option<Dispatcher<'d, 'e,>,>,
}

impl<'d, 'e,> SimpleState for Simulation<'d, 'e,> {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);

        self.initialise_camera(world);
        let sprite_sheet = self.load_sprite_sheet(world);
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0, // ant is the first sprite in the sprite_sheet
        };

        world.register::<Colony>();
        world.register::<Ant>();
        world.register::<SpriteRender>();
        for _ in 0..COLONY_COUNT {
            let x = rand::thread_rng().gen_range(0, FIELD_WIDTH);
            let y = rand::thread_rng().gen_range(0, FIELD_HEIGHT);
            let home = Coordinate::new(x, y);
            let mut transform = Transform::default();
            transform.set_xyz(home.x as f32, home.y as f32, 0.0);
            world.create_entity()
                .with(Colony::new(home, 1000u32))
                .with(SpriteRender {
                    sprite_sheet: sprite_sheet.clone(),
                    sprite_number: 1,
                })
                .with(transform)
                .build();
        }
        
        world.register::<Food>();
        for _ in 0..FOOD_COUNT {
            let x = rand::thread_rng().gen_range(0, FIELD_WIDTH);
            let y = rand::thread_rng().gen_range(0, FIELD_HEIGHT);
            let pos = Coordinate::new(x, y);
            let mut transform = Transform::default();
            transform.set_xyz(pos.x as f32, pos.y as f32, 0.0);
            world.create_entity()
                .with(Food::new(pos))
                .with(SpriteRender {
                    sprite_sheet: sprite_sheet.clone(),
                    sprite_number: 3,
                })
                .with(transform)
                .build();
            
            field.place_food_by_pos(pos);
        }

        world.add_resource(field);
        

        self.main_dispatcher = Some({
            let mut dispatcher = DispatcherBuilder::new()
                .with(SpawnAnts::new(sprite_render.clone()), "spawn_ants_system", &[],)
                .with(CheckCells {}, "check_cells_system", &["spawn_ants_system"])
                .with(MoveAnts {}, "move_ants_system", &["spawn_ants_system", "check_cells_system"])
                .build();

            dispatcher.setup(&mut world.res,);

            dispatcher
        },);
    }
    
    /// Executed repeatedly at stable, predictable intervals (1/60th of a second by default).
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit).
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.tick(data);
        data.data.update(&data.world);
        Trans::None
    }

}

impl<'d, 'e,> Simulation<'d, 'e,> {
    
    pub fn new() -> Simulation<'d, 'e,> {
        Simulation {
            main_dispatcher: None,
        }
    }

    fn initialise_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        transform.set_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,
                FIELD_WIDTH as f32,
                0.0,
                FIELD_HEIGHT as f32,
            )))
            .with(transform)
            .build();
    }

    fn load_sprite_sheet(&self, world: &mut World) -> SpriteSheetHandle {
        // Load the sprite sheet necessary to render the graphics.
        // The texture is the pixel data
        // `sprite_sheet` is the layout of the sprites on the image
        // `texture_handle` is a cloneable reference to the texture
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "textures/ant.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "textures/ant.ron", // Here we load the associated ron file
            SpriteSheetFormat,
            texture_handle, // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        )
    }

    fn tick(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) {
        if let Some(disp) = &mut self.main_dispatcher {
            let world = &_data.world;
            disp.dispatch(&world.res);
            _data.world.maintain();
        }
    }
}
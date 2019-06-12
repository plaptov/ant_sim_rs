use crate::internals::{
    field::Field,
    coordinate::Coordinate,
};
use crate::components::{
    colony::Colony,
    food::Food,
};
use crate::drawing::field_image;

use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::graphics::{self, *};
use ggez::mint::Point2;
use rand::Rng;

pub const FIELD_WIDTH: i32 = 800;
pub const FIELD_HEIGHT: i32 = 400;
pub const COLONY_COUNT: i32 = 5;
pub const FOOD_COUNT: u32 = 20;

pub struct Simulation {
    field: Field,
    colonies: Vec<Colony>,
    ant_mesh: Mesh,
    food_mesh: Mesh,
    pheromones_mesh: Mesh,
    field_img: Canvas,
}

impl Simulation {

    pub fn new(ctx: &mut Context) -> GameResult<Simulation> {
        let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);

        let mut colonies = vec!{};
        for _ in 0..COLONY_COUNT {
            let x = rand::thread_rng().gen_range(0, FIELD_WIDTH);
            let y = rand::thread_rng().gen_range(0, FIELD_HEIGHT);
            let home = Coordinate::new(x, y);
            colonies.push(Colony::new(home, 1000usize));
        }
        
        let mut foods = vec!{};
        for _ in 0..FOOD_COUNT {
            let x = rand::thread_rng().gen_range(0, FIELD_WIDTH);
            let y = rand::thread_rng().gen_range(0, FIELD_HEIGHT);
            let pos = Coordinate::new(x, y);
            foods.push(Food::new(pos));
            field.place_food_by_pos(pos);
        }

        let opt = StrokeOptions::default();
        let ant_mesh = Mesh::new_rectangle(ctx, DrawMode::Stroke(opt), Rect::new(0.0, 0.0, 1.0, 1.0), Color::from_rgb(0, 0, 255))?;
        let food_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(-1.0, -1.0, 3.0, 3.0), Color::from_rgb(0, 150, 0))?;
        let pheromones_mesh = Mesh::new_rectangle(ctx, DrawMode::Stroke(opt), Rect::new(0.0, 0.0, 1.0, 1.0), Color::from_rgb(255, 255, 50))?;
        let field_img = field_image::make_field_image(ctx, &field)?;

        Ok(Simulation {
            field,
            colonies,
            ant_mesh,
            food_mesh,
            pheromones_mesh,
            field_img,
        })
    }

    fn check_cells(&mut self) {
        for colony in &mut self.colonies {
            colony.check_cells(&mut self.field);
        }
    }

    fn move_ants(&mut self) {
        for colony in &mut self.colonies {
            colony.move_ants(&mut self.field);
        }
    }
    
}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.check_cells();
        self.move_ants();
        Ok(())
    }

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::yield_now()`
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &self.field_img, DrawParam::default())?;

        for cell in self.field.get_cells() {
            let mesh;

            if cell.food > 0 {
                mesh = &self.food_mesh;
            }
            else if cell.ants > 0 {
                mesh = &self.ant_mesh;
            }
            else if cell.pheromones > 0 {
                mesh = &self.pheromones_mesh;
            }
            else {
                continue;
            }

            graphics::draw(ctx, mesh, DrawParam {
                dest: Point2 { x: cell.position.x as f32, y: cell.position.y as f32 },
                //color,
                .. Default::default()
            })?;
        }

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}
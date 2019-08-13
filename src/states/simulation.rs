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
use ggez::graphics::{self, *, spritebatch::SpriteBatch};
use ggez::mint::{Point2, Vector2};
use rand::Rng;

pub const FIELD_WIDTH: i32 = 800;
pub const FIELD_HEIGHT: i32 = 400;
pub const COLONY_COUNT: i32 = 5;
pub const FOOD_COUNT: u32 = 20;

pub struct Simulation {
    field: Field,
    colonies: Vec<Colony>,
    ants_batch: SpriteBatch,
    food_mesh: Mesh,
    pheromones_batch: SpriteBatch,
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

        let ant_image = graphics::Image::new(ctx, "/ant.png")?;
        let ants_batch = graphics::spritebatch::SpriteBatch::new(ant_image);
        let pheromone_image = graphics::Image::new(ctx, "/pheromone.png")?;
        let pheromones_batch = graphics::spritebatch::SpriteBatch::new(pheromone_image);

        let food_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(-4.0, -4.0, 9.0, 9.0), Color::from_rgb(0, 150, 0))?;
        let field_img = field_image::make_field_image(ctx, &field)?;

        Ok(Simulation {
            field,
            colonies,
            ants_batch,
            food_mesh,
            pheromones_batch,
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
        let params = DrawParam::new().scale(Vector2 { x: 2.0, y: 2.0 });

        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &self.field_img, params)?;

        for cell in self.field.get_cells() {
            let point = Point2 { x: cell.position.x as f32, y: cell.position.y as f32 };
            let cur_params = params.dest(point);

            if cell.food > 0 {
                graphics::draw(ctx, &self.food_mesh, cur_params)?;
            }
            else if cell.ants > 0 {
                self.ants_batch.add(cur_params);
            }
            else if cell.pheromones > 0 {
                self.pheromones_batch.add(cur_params);
            }
        }

        graphics::draw(ctx, &self.ants_batch, params)?;
        self.ants_batch.clear();
        graphics::draw(ctx, &self.pheromones_batch, params)?;
        self.pheromones_batch.clear();
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}
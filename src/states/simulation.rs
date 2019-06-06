use crate::internals::{
    field::Field,
    coordinate::Coordinate,
};
use crate::components::{
    colony::Colony,
    ant::*,
    food::Food,
};
use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::graphics::{self, *};
use rand::Rng;

pub const FIELD_WIDTH: i32 = 800;
pub const FIELD_HEIGHT: i32 = 400;
pub const COLONY_COUNT: i32 = 5;
pub const FOOD_COUNT: u32 = 20;

pub struct Simulation {
    field: Field,
    colonies: Vec<Colony>,
    foods: Vec<Food>,
    ants: Vec<Ant>,
    pixel: Mesh,
}

impl Simulation {

    pub fn new(ctx: &mut Context) -> GameResult<Simulation> {
        graphics::set_background_color(ctx, graphics::WHITE);

        let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);

        let mut colonies = vec!{};
        for _ in 0..COLONY_COUNT {
            let x = rand::thread_rng().gen_range(0, FIELD_WIDTH);
            let y = rand::thread_rng().gen_range(0, FIELD_HEIGHT);
            let home = Coordinate::new(x, y);
            colonies.push(Colony::new(home, 1000u32));
        }
        
        let mut foods = vec!{};
        for _ in 0..FOOD_COUNT {
            let x = rand::thread_rng().gen_range(0, FIELD_WIDTH);
            let y = rand::thread_rng().gen_range(0, FIELD_HEIGHT);
            let pos = Coordinate::new(x, y);
            foods.push(Food::new(pos));
            field.place_food_by_pos(pos);
        }

        let ants = vec!{};

        let pixel = Mesh::new_polyline(ctx, DrawMode::Line(1.0), &[Point2::new(1.0, 1.0), Point2::new(1.0, 1.0)])?;

        Ok(Simulation {
            field,
            colonies,
            foods,
            ants,
            pixel,
        })
    }

    fn check_cells(&mut self) {
        for ant in &mut self.ants {
            ant.check_current_cell(&mut self.field);
        }
    }

    fn spawn_ants(&mut self) {
        for colony in &mut self.colonies {
                if colony.ants_count < colony.max_ants {
                    let ant = colony.make_ant();
                    self.ants.push(ant);
                }
            }
    }

    fn move_ants(&mut self) {
        for ant in &mut self.ants {
            ant.make_move(&mut self.field);
        }
    }
    
}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.check_cells();
        self.spawn_ants();
        self.move_ants();
        Ok(())
    }

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::yield_now()`
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for cell in self.field.get_cells() {
            let mut color = None;

            if cell.is_obstacle {
                color = Some(graphics::BLACK);
            }
            else if cell.food > 0 {
                color = Some(Color::from_rgb(0, 0x64, 0));
            }
            else if cell.ants > 0 {
                color = Some(Color::from_rgb(0, 0, 0xFF));
            }
            else if cell.pheromones > 0 {
                color = Some(Color::from_rgb(0xFF, 0xFF, 0));
            }

            graphics::draw_ex(ctx, &self.pixel, DrawParam {
                dest: Point2::new(cell.position.x as f32, cell.position.y as f32),
                color,
                .. Default::default()
            })?;
        }

        graphics::present(ctx);
        ggez::timer::yield_now();
        Ok(())
    }
}
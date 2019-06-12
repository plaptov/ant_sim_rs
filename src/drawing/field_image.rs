use crate::internals::{
    field::Field,
};

use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas};
use ggez::mint::Point2;

pub fn make_field_image(ctx: &mut Context, field: &Field) -> GameResult<Canvas> {
    let image = graphics::Image::new(ctx, "/black_pixel.png")?;
    let canva = graphics::Canvas::with_window_size(ctx)?;

    graphics::set_canvas(ctx, Some(&canva));
    graphics::clear(ctx, graphics::WHITE);

    let mut spritebatch = graphics::spritebatch::SpriteBatch::new(image);
    let mut cnt = 0i32;

    for cell in field.get_cells() {
        if cell.is_obstacle {
            cnt += 1;
            spritebatch.add(graphics::DrawParam::new()
                .dest(Point2 { x: cell.position.x as f32, y: cell.position.y as f32 })
                .color(graphics::BLACK)
            );
        }
    }

    println!("{}", cnt);
    graphics::draw(ctx, &spritebatch, graphics::DrawParam::new())?;
    graphics::set_canvas(ctx, None);
    Ok(canva)
}
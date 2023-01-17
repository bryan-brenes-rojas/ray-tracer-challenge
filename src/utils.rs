use std::fs::File;
use rand::Rng;
use crate::canvas::Canvas;
use crate::color::Color;
use std::io::{Result, Write};

#[allow(dead_code)]
pub const EPSILON: f32 = 0.00001;

pub fn write_to_file(path: &str, content: &str) -> Result<()>{
    let mut file = File::create(path)?;
    file.write(content.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

#[allow(dead_code)]
pub fn paint_square(canvas: &mut Canvas, x: i32, y: i32, size: i32) {
    let initial_pos_x = x - size;
    let initial_pos_y = y - size;
    let final_pos_x = initial_pos_x + (2 * size + 1);
    let final_pos_y = initial_pos_y + (2 * size + 1);

    let color = get_random_color();
    for row_index in initial_pos_y..final_pos_y {
        for col_index in initial_pos_x..final_pos_x {
            canvas.write_pixel(&color, col_index as usize, row_index as usize);
        }
    }
}

mod canvas;
mod color;
mod point;
mod utils;
mod vector;

use canvas::Canvas;
use color::Color;
use point::Point;
use vector::Vector;

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Env {
    gravity: Vector,
    wind: Vector,
}

fn main() {
    let mut canvas = Canvas::new(350, 350);

    let mut projectile = Projectile {
        position: Point::new(0.0, 0.0, 0.0),
        velocity: Vector::new(0.7, 1.8, 0.0),
    };
    let env = Env {
        gravity: Vector::new(0.0, -0.09, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while projectile.position.y >= 0.0 {
        projectile = tick(&env, &projectile);
        println!("Current position {:?}", projectile.position);
        paint_square(
            &mut canvas,
            (projectile.position.x * 15.0) as usize,
            350 - (projectile.position.y * 15.0) as usize,
            2,
        )
    }
    canvas.to_ppm();
}

fn tick(env: &Env, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}

fn paint_square(canvas: &mut Canvas, x: usize, y: usize, size: usize) {
    let initial_pos_x = (x - size).clamp(0, 350);
    let initial_pos_y = (y - size).clamp(0, 350);
    let final_pos_x = initial_pos_x + (2 * size + 1);
    let final_pos_y = initial_pos_y + (2 * size + 1);

    for row_index in initial_pos_y..final_pos_y {
        for col_index in initial_pos_x..final_pos_x {
            canvas.write_pixel(&Color::new(1.0, 0.0, 0.0), col_index, row_index);
        }
    }
}

mod canvas;
mod color;
mod matrix;
mod point;
mod utils;
mod vector;

use canvas::Canvas;
use color::Color;
use matrix::*;
use point::Point;
use rand::Rng;
use vector::Vector;
#[allow(dead_code)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[allow(dead_code)]
struct Env {
    gravity: Vector,
    wind: Vector,
}

fn main() {
    let m = Matrix::scaling_3d(2.0, 3.0, 4.0);
    let m_inverse = m.inverse();

    let p = Point::new(-4.0, 6.0, 8.0);
    let new_point = &m * &p;

    let restoring_point = &m_inverse * &new_point;
    println!("new point: {:#?}", new_point);
    println!("restoration: {:#?}", restoring_point);
}

#[allow(dead_code)]
fn projectile_drawing() {
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
        paint_square(
            &mut canvas,
            (projectile.position.x * 15.0) as i32,
            350 - (projectile.position.y * 15.0) as i32,
            3,
        );
        projectile = tick(&env, &projectile);
    }
    canvas.to_ppm();
}

#[allow(dead_code)]
fn tick(env: &Env, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}

#[allow(dead_code)]
fn paint_square(canvas: &mut Canvas, x: i32, y: i32, size: i32) {
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

#[allow(dead_code)]
fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

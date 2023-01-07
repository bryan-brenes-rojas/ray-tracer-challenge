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
    let mut canvas = Canvas::new(100, 100);
    canvas.write_pixel(&Color::new(1.0, 0.5, 0.3), 0, 0);
    canvas.write_pixel(&Color::new(1.0, 0.5, 0.3), 100, 100);
    canvas.to_ppm();

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0),
    };
    let env = Env {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        projectile = tick(&env, &projectile);
        println!("Current position {:?}", projectile.position);
    }
}

fn tick(env: &Env, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}

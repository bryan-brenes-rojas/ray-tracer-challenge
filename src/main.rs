mod point;
mod utils;
mod vector;
mod color;

use point::Point;
use vector::Vector;
use color::Color;

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Env {
    gravity: Vector,
    wind: Vector,
}

fn main() {
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

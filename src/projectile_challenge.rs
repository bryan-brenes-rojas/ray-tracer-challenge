use crate::canvas::Canvas;
use crate::vector::Vector;
use crate::point::Point;
use crate::utils::paint_square;

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

#[allow(dead_code)]
pub fn projectile_drawing() {
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

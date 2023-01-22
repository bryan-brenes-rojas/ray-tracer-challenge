mod canvas;
mod clock_challenge;
mod color;
mod intersection;
mod matrix;
mod object;
mod point;
mod projectile_challenge;
mod ray;
mod sphere;
mod utils;
mod vector;

use std::f32::consts::PI;

use canvas::Canvas;
use point::Point;
use ray::Ray;
use sphere::Sphere;
use utils::paint_square;
use vector::Vector;

use crate::matrix::Matrix;

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let ray_num = 200;
    let sphere = Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        200.0,
        Some(Matrix::translation_3d(500.0, 500.0, 0.0)),
    );
    let angle_step = (PI / 2.0) / ray_num as f32;
    for i in 0..(ray_num + 1) {
        let original_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let ray_rotation_transform = Matrix::rotate_z_3d(-(i as f32 * angle_step));
        let ray = original_ray.transform(&ray_rotation_transform);
        paint_square(
            &mut canvas,
            (ray.direction.x * 300.0) as i32,
            (ray.direction.y * 300.0) as i32,
            0,
        );
        let intersections = sphere.intersections(&ray);
        match sphere.hit(&intersections) {
            Some(intersection) => {
                let p = &ray.origin + &(&ray.direction * intersection.t);
                paint_square(&mut canvas, p.x as i32, p.y as i32, 0);
            }
            None => (),
        }
    }

    let sphere_origin = &sphere.transform * &sphere.origin;
    paint_square(&mut canvas, sphere_origin.x as i32, sphere_origin.y as i32, 5);
    canvas.to_ppm();
}

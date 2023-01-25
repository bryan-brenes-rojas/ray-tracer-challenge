use std::f32::consts::PI;

use matrix::Matrix;
use object::Object;
use point::Point;
use sphere::Sphere;

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
mod sphere_silhouette;
mod utils;
mod vector;

fn main() {
    let sphere = Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        1.0,
        Some(&Matrix::scaling_3d(1.0, 0.5, 1.0) * &Matrix::rotate_z_3d(PI / 5.0)),
    );
    let normal = sphere.normal_at(&Point::new(0.0, 2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0));
    println!("{:#?}", normal)
}

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

use point::Point;
use ray::Ray;
use sphere::Sphere;
use vector::Vector;

use crate::matrix::Matrix;

fn main() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        1.0,
        Some(Matrix::scaling_3d(2.0, 2.0, 2.0)),
    );
    let i = sphere.intersections(&ray);
    for int in i {
        println!("{:#?}", int.t);
    }
    // println!("{:#?}", i.into_iter().map(|i| println!("{:#?}",i)));
}

mod canvas;
mod clock_challenge;
mod color;
mod matrix;
mod point;
mod projectile_challenge;
mod ray;
mod sphere;
mod utils;
mod vector;
mod intersection;
mod object;

use point::Point;
use vector::Vector;
use sphere::Sphere;
use ray::Ray;

fn main() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
    let i = sphere.intersections(&ray);
    let hit = sphere.hit(&i);
    println!("{:#?}", hit);
}

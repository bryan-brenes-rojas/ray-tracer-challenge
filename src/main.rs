mod vector;
mod point;

use point::Point;
use vector::Vector;

fn main() {
    let p1 = Point::new(0, 0, 30);
    let v1 = Vector::new(0, 0, 30);
    println!("Hello, world! {:#?}, {:#?}, {:?}", p1, v1, p1.x);
}

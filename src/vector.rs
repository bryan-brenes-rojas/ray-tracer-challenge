use std::ops::Add;
use std::ops::Sub;
use crate::point::Point;

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }
}

// vector + vector
impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 0.0,
        }
    }
}

// vector + point
impl Add<&Point> for &Vector {
    type Output = Point;

    fn add(self, other: &Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 1.0,
        }
    }
}

// vector - vector
impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_w_0_by_default() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn should_be_able_to_create_vectors() {
        let vector = Vector::new(2.0, -4.0, 5.0);
        assert_eq!(vector.x, 2.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, 5.0);
    }
}

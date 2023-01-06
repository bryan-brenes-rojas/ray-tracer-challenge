use crate::point::Point;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

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

// -vector
impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(0.0 - self.x, 0.0 - self.y, 0.0 - self.z)
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

    #[test]
    fn vector_plus_vector_is_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let new_vector = &v1 + &v2;
        assert_eq!(new_vector.x, 2.0);
        assert_eq!(new_vector.y, 4.0);
        assert_eq!(new_vector.z, 6.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn vector_plus_point_is_point() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let new_point = &v + &p;
        assert_eq!(new_point.x, 2.0);
        assert_eq!(new_point.y, 4.0);
        assert_eq!(new_point.z, 6.0);
        assert_eq!(new_point.w, 1.0);
    }

    #[test]
    fn vector_minus_vector_is_point() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 4.0, 6.0);
        let new_vector = &v1 - &v2;
        assert_eq!(new_vector.x, -1.0);
        assert_eq!(new_vector.y, -2.0);
        assert_eq!(new_vector.z, -3.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn should_be_able_to_negate_a_vector() {
        let v = Vector::new(1.0, 0.0, 3.0);
        let new_vector = -&v;
        assert_eq!(new_vector.x, -1.0);
        assert_eq!(new_vector.y, 0.0);
        assert_eq!(new_vector.z, -3.0);
        assert_eq!(new_vector.w, 0.0);
    }
}

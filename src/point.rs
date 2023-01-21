use crate::matrix::Matrix;
use crate::vector::Vector;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z, w: 1.0 }
    }

    pub fn to_matrix(&self) -> Matrix {
        let mut m = Matrix::new(4, 1);
        m.patch(vec![vec![self.x], vec![self.y], vec![self.z], vec![self.w]]);
        m
    }
}

// point + vector
impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, other: &Vector) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 1.0,
        }
    }
}

// point - point
impl Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, other: &Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0,
        }
    }
}

// point - vector
impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, other: &Vector) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_w_1_by_default() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn should_be_able_to_create_points() {
        let point = Point::new(2.0, -4.0, 5.0);
        assert_eq!(point.x, 2.0);
        assert_eq!(point.y, -4.0);
        assert_eq!(point.z, 5.0);
    }

    #[test]
    fn point_plus_vector_is_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        let new_point = &p + &v;
        assert_eq!(new_point.x, 2.0);
        assert_eq!(new_point.y, 4.0);
        assert_eq!(new_point.z, 6.0);
        assert_eq!(new_point.w, 1.0);
    }

    #[test]
    fn point_minus_point_is_vector() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        let new_vector = &p1 - &p2;
        assert_eq!(new_vector.x, 0.0);
        assert_eq!(new_vector.y, 0.0);
        assert_eq!(new_vector.z, 0.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn point_minus_vector_is_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(2.0, 4.0, 6.0);
        let new_point = &p - &v;
        assert_eq!(new_point.x, -1.0);
        assert_eq!(new_point.y, -2.0);
        assert_eq!(new_point.z, -3.0);
        assert_eq!(new_point.w, 1.0);
    }
}

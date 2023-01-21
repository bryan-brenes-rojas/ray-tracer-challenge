use crate::{point::Point, vector::Vector};

#[allow(dead_code)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point {
        &self.origin + &(&self.direction * t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_a_ray() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(ray.origin.x, 2.0);
        assert_eq!(ray.origin.y, 3.0);
        assert_eq!(ray.origin.z, 4.0);
        assert_eq!(ray.origin.w, 1.0);

        assert_eq!(ray.direction.x, 1.0);
        assert_eq!(ray.direction.y, 0.0);
        assert_eq!(ray.direction.z, 0.0);
        assert_eq!(ray.direction.w, 0.0);
    }

    #[test]
    fn should_create_a_rays_position() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        let p1 = ray.position(0.0);
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 3.0);
        assert_eq!(p1.z, 4.0);
        assert_eq!(p1.w, 1.0);

        let p2 = ray.position(1.0);
        assert_eq!(p2.x, 3.0);
        assert_eq!(p2.y, 3.0);
        assert_eq!(p2.z, 4.0);
        assert_eq!(p2.w, 1.0);

        let p3 = ray.position(-1.0);
        assert_eq!(p3.x, 1.0);
        assert_eq!(p3.y, 3.0);
        assert_eq!(p3.z, 4.0);
        assert_eq!(p3.w, 1.0);
    }
}

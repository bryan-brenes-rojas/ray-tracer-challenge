use crate::{point::Point, vector::Vector, matrix::Matrix};

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

    pub fn transform(&self, matrix:&Matrix) -> Self {
        let new_origin = matrix * &self.origin;
        let new_dir = matrix * &self.direction;
        Ray::new(new_origin, new_dir)
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

    #[test]
    fn should_translate_the_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let matrix = Matrix::translation_3d(3.0, 4.0, 5.0);
        let new_ray = ray.transform(&matrix);
        assert_eq!(new_ray.origin.x, 4.0);
        assert_eq!(new_ray.origin.y, 6.0);
        assert_eq!(new_ray.origin.z, 8.0);
        assert_eq!(new_ray.origin.w, 1.0);

        assert_eq!(new_ray.direction.x, 0.0);
        assert_eq!(new_ray.direction.y, 1.0);
        assert_eq!(new_ray.direction.z, 0.0);
        assert_eq!(new_ray.direction.w, 0.0);
    }

    #[test]
    fn should_scale_the_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let matrix = Matrix::scaling_3d(2.0, 3.0, 4.0);
        let new_ray = ray.transform(&matrix);
        assert_eq!(new_ray.origin.x, 2.0);
        assert_eq!(new_ray.origin.y, 6.0);
        assert_eq!(new_ray.origin.z, 12.0);
        assert_eq!(new_ray.origin.w, 1.0);

        assert_eq!(new_ray.direction.x, 0.0);
        assert_eq!(new_ray.direction.y, 3.0);
        assert_eq!(new_ray.direction.z, 0.0);
        assert_eq!(new_ray.direction.w, 0.0);
    }
}

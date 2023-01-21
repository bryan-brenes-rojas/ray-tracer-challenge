use std::cmp::Ordering;

use crate::{intersection::Intersection, matrix::Matrix, object::Object, point::Point, ray::Ray};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f32,
    pub transform: Matrix,
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        Sphere {
            origin: self.origin,
            radius: self.radius,
            transform: self.transform.clone(),
        }
    }
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(origin: Point, radius: f32, transform: Option<Matrix>) -> Self {
        let t = match transform {
            Some(t) => t,
            None => Matrix::identity(4),
        };
        Sphere {
            origin,
            radius,
            transform: t,
        }
    }

    /// Always return the intersections vector sorted
    pub fn intersections(&self, ray: &Ray) -> Vec<Intersection<Sphere>> {
        let mut intersections: Vec<Intersection<Sphere>> = vec![];
        let transformed_ray = ray.transform(&self.transform.inverse());

        let sphere_to_ray = &transformed_ray.origin - &self.origin;
        let a = transformed_ray
            .direction
            .dot_product(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot_product(&sphere_to_ray);
        let c = &sphere_to_ray.dot_product(&sphere_to_ray) - (&self.radius.powf(2.0));
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            intersections.push(Intersection::new(t1, self.clone()));
            intersections.push(Intersection::new(t2, self.clone()));
        }
        intersections.sort_by(|a, b| {
            if a.t < b.t {
                Ordering::Less
            } else if a.t > b.t {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        intersections
    }

    pub fn hit<'a>(
        &self,
        intersections: &'a Vec<Intersection<Sphere>>,
    ) -> Option<&'a Intersection<Sphere>> {
        for intersec in intersections {
            if intersec.t >= 0.0 {
                return Some(&intersec);
            }
        }
        None
    }
}

impl Object for Sphere {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn should_create_sphere() {
        let s = Sphere::new(Point::new(1.0, 2.0, 3.0), 1.0, None);
        assert_eq!(s.origin.x, 1.0);
        assert_eq!(s.origin.y, 2.0);
        assert_eq!(s.origin.z, 3.0);
        assert_eq!(s.radius, 1.0);
    }

    #[test]
    fn should_get_intersection_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, None);
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);
    }

    #[test]
    fn should_get_intersection_tangent_points() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, None);
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
    }

    #[test]
    fn should_get_intersection_missing_points() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, None);
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn should_get_intersection_inside_sphere_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, None);
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);
    }

    #[test]
    fn should_get_intersection_behind_sphere_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, None);
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);
    }

    #[test]
    fn should_get_intersection_with_transform() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            1.0,
            Some(Matrix::scaling_3d(2.0, 2.0, 2.0)),
        );
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn should_get_intersection_with_transform_2() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            1.0,
            Some(Matrix::translation_3d(5.0, 0.0, 0.0)),
        );
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 0);
    }
}

use crate::object::Object;

#[derive(Debug)]
pub struct Intersection<T: Object> {
    pub t: f32,
    pub object: T,
}

impl<T: Object> Intersection<T> {
    pub fn new(t: f32, object: T) -> Intersection<T> {
        Intersection { t, object }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point::Point, sphere::Sphere};

    #[test]
    fn should_create_new_intersection() {
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let intersection = Intersection::new(3.0, sphere);
        assert_eq!(intersection.t, 3.0);
        assert_eq!(intersection.object.radius, 1.0);
    }
}

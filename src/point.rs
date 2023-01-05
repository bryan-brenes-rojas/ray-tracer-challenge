#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z, w: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_w_1_by_default() {
        let point = Point::new(1, 2, 3);
        assert_eq!(point.w, 1);
    }

    #[test]
    fn should_be_able_to_create_points() {
        let point = Point::new(2, -4, 5);
        assert_eq!(point.x, 2);
        assert_eq!(point.y, -4);
        assert_eq!(point.z, 5);
    }
}

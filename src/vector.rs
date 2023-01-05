#[derive(Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x, y, z, w: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_w_0_by_default() {
        let vector = Vector::new(1,2,3);
        assert_eq!(vector.w, 0);
    }

    #[test]
    fn should_be_able_to_create_vectors() {
        let vector = Vector::new(2, -4, 5);
        assert_eq!(vector.x, 2);
        assert_eq!(vector.y, -4);
        assert_eq!(vector.z, 5);
    }
}

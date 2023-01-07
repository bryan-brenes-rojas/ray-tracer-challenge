use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
}

impl Add<&Color> for &Color {
    type Output = Color;

    fn add(self, other: &Color) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub<&Color> for &Color {
    type Output = Color;

    fn sub(self, other: &Color) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f32> for &Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Self::Output {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_create_a_color() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(color.r, -0.5);
        assert_eq!(color.g, 0.4);
        assert_eq!(color.b, 1.7);
    }

    #[test]
    fn should_be_able_to_add_colors() {
        let c1 = Color::new(1.0, 2.0, 3.0);
        let c2 = Color::new(1.0, 2.0, 3.0);
        let new_color = &c1 + &c2;

        assert_eq!(new_color.r, 2.0);
        assert_eq!(new_color.g, 4.0);
        assert_eq!(new_color.b, 6.0);
    }

    #[test]
    fn should_be_able_to_subtract_colors() {
        let c1 = Color::new(1.0, 2.0, 3.0);
        let c2 = Color::new(1.0, 3.0, 1.0);
        let new_color = &c1 - &c2;

        assert_eq!(new_color.r, 0.0);
        assert_eq!(new_color.g, -1.0);
        assert_eq!(new_color.b, 2.0);
    }

    #[test]
    fn should_be_able_to_multiply_a_color_by_scalar() {
        let c1 = Color::new(1.0, 2.0, 3.0);
        let new_color = &c1 * 3.0;

        assert_eq!(new_color.r, 3.0);
        assert_eq!(new_color.g, 6.0);
        assert_eq!(new_color.b, 9.0);
    }

    #[test]
    fn should_be_able_to_multiply_a_color_with_color() {
        let c1 = Color::new(1.0, 2.0, 3.0);
        let c2 = Color::new(1.0, 3.0, 1.0);
        let new_color = &c1 * &c2;

        assert_eq!(new_color.r, 1.0);
        assert_eq!(new_color.g, 6.0);
        assert_eq!(new_color.b, 3.0);
    }
}

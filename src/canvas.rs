use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Self {
        Canvas {
            width: w,
            height: h,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); usize::from(w)]; usize::from(h)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_create_a_canvas() {
        let c = Canvas::new(10, 20);
        println!("{:#?}", c.pixels);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixels.len(), 20);
        assert_eq!(c.pixels[0].len(), 10);
    }
}

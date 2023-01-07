use crate::color::Color;
use crate::utils::write_to_file;

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

    pub fn write_pixel(&mut self, color: &Color, x: usize, y: usize) {
        let x_coord = x.clamp(0, usize::from(self.width - 1));
        let y_coord = y.clamp(0, usize::from(self.height - 1));
        self.pixels[y_coord][x_coord] = *color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let x_coord = x.clamp(0, usize::from(self.width - 1));
        let y_coord = y.clamp(0, usize::from(self.height - 1));
        self.pixels[y_coord][x_coord]
    }

    pub fn to_ppm(&self) {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                s.push_str(&pixel.scale(255.0).to_string());
                s.push_str(" ");
            }
            s.push_str("\n");
        }
        match write_to_file("test.ppm", &s) {
            Ok(_) => println!("File written successfully"),
            Err(e) => println!("{:?}", e),
        };
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

    #[test]
    fn should_be_able_to_write_a_pixel() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(&Color::new(1.0, 0.5, 0.3), 0, 0);
        let mut color_written = c.get_pixel(0, 0);
        assert_eq!(color_written.r, 1.0);
        assert_eq!(color_written.g, 0.5);
        assert_eq!(color_written.b, 0.3);

        c.write_pixel(&Color::new(1.0, 0.5, 0.3), 2, 0);
        color_written = c.get_pixel(2, 0);
        assert_eq!(color_written.r, 1.0);
        assert_eq!(color_written.g, 0.5);
        assert_eq!(color_written.b, 0.3);
    }

    #[test]
    fn should_clamp_values_greater_than_dimensions_write() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(&Color::new(1.0, 0.5, 0.3), 200, 200);
        let p = c.get_pixel(9, 19);
        assert_eq!(p.r, 1.0);
        assert_eq!(p.g, 0.5);
        assert_eq!(p.b, 0.3);
    }

    #[test]
    fn should_clamp_values_greater_than_dimensions_get() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(&Color::new(1.0, 0.5, 0.3), 9, 19);
        let p = c.get_pixel(200, 200);
        assert_eq!(p.r, 1.0);
        assert_eq!(p.g, 0.5);
        assert_eq!(p.b, 0.3);
    }
}

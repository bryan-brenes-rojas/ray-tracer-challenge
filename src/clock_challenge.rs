use crate::canvas::Canvas;
use crate::point::Point;
use crate::matrix::Matrix;
use std::f32::consts::PI;
use crate::utils::paint_square;

pub fn clock_drawing() {
    let mut canvas = Canvas::new(1000, 1000);
    let clock_width = 200.0;
    let ticks = 12.0;
    let angle_step = (2.0 * PI) / ticks;

    for i in 0..ticks as i32 {
        let p = Point::new(clock_width, 0.0, 0.0);
        let rotation = Matrix::rotate_z_3d(angle_step * i as f32);
        let translation = Matrix::translation_3d(500.0, 500.0, 0.0);
        let new_point = &(&translation * &rotation) * &p;
        paint_square(&mut canvas, new_point.x as i32, new_point.y as i32, 5);
    }
    canvas.to_ppm();
}

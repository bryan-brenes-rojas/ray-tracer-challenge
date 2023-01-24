use crate::{
    canvas::Canvas, matrix::Matrix, point::Point, ray::Ray, sphere::Sphere, utils::paint_square,
    vector::Vector,
};
use std::f32::consts::PI;

fn create_ray(x: usize, y: usize, origin: &Point) -> Ray {
    let pixel_point = Point::new(x as f32, y as f32, 0.0);
    let direction = (&pixel_point - origin).normalize();
    let ray = Ray::new(*origin, direction);
    ray
}

#[allow(dead_code)]
pub fn draw_sphere_silhouette() {
    let mut canvas = Canvas::new(1000, 1000);
    let ray_num = 200;
    let sphere = Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        200.0,
        Some(Matrix::translation_3d(500.0, 500.0, 0.0)),
    );
    let angle_step = (PI / 2.0) / ray_num as f32;
    for i in 0..(ray_num + 1) {
        let original_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let ray_rotation_transform = Matrix::rotate_z_3d(-(i as f32 * angle_step));
        let ray = original_ray.transform(&ray_rotation_transform);
        paint_square(
            &mut canvas,
            (ray.direction.x * 300.0) as i32,
            (ray.direction.y * 300.0) as i32,
            0,
        );
        let intersections = sphere.intersections(&ray);
        match sphere.hit(&intersections) {
            Some(intersection) => {
                let p = &ray.origin + &(&ray.direction * intersection.t);
                paint_square(&mut canvas, p.x as i32, p.y as i32, 0);
            }
            None => (),
        }
    }

    let sphere_origin = &sphere.transform * &sphere.origin;
    paint_square(
        &mut canvas,
        sphere_origin.x as i32,
        sphere_origin.y as i32,
        5,
    );
    canvas.to_ppm();
}

pub fn draw_sphere() {
    let canvas_width: usize = 500;
    let mut canvas = Canvas::new(canvas_width as u16, canvas_width as u16);
    let ray_origin = Point::new(canvas_width as f32 / 2.0, canvas_width as f32 / 2.0, 420.0);
    let sphere = Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        200.0,
        Some(Matrix::translation_3d(
            canvas_width as f32 / 2.0,
            canvas_width as f32 / 2.0,
            0.0,
        )),
    );
    for r_i in 0..canvas_width {
        for c_i in 0..canvas_width {
            let ray = create_ray(c_i, r_i, &ray_origin);
            let intersections = sphere.intersections(&ray);
            match sphere.hit(&intersections) {
                Some(intersection) => {
                    let p = &ray.origin + &(&ray.direction * intersection.t);
                    paint_square(&mut canvas, p.x as i32, p.y as i32, 0);
                }
                None => (),
            }
        }
    }
    canvas.to_ppm();
}

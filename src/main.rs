#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod objects;
mod ray;
mod vec3;

use image::{ImageBuffer, Rgb, RgbImage};
use objects::{Object, Sphere};
use vec3::Vec3;

use crate::ray::Ray;

type BoxedObject = Box<dyn Object>;

#[must_use]
pub fn color(ray: &Ray, world: &[BoxedObject]) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0..std::f32::MAX) {
        let normal = hit.normal;
        return 0.5 * (normal + Vec3(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction.into_unit();
    let t = 0.5 * (unit_direction.1 + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const AR: f32 = 16.0 / 9.0;
    const HEIGHT: usize = 2160;
    const WIDTH: usize = (HEIGHT as f32 * AR) as usize;

    // World
    let world: Vec<BoxedObject> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    //Camera
    let viewport_height = 2.0;
    let viewport_width = AR * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // Render
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u = x as f32 / (WIDTH - 1) as f32;
        let v = 1.0 - (y as f32 / (HEIGHT - 1) as f32);
        let r = Ray {
            origin,
            direction: lower_left_corner + u * horizontal + v * vertical - origin,
        };

        let color = color(&r, &world).to_rgb();
        *pixel = Rgb(color);
    }
    buffer
        .save("renders/world.png")
        .expect("failed to save image.");
}

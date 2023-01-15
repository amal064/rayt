use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, viewport_height: f32, aspect_ratio: f32, focal_length: f32) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        let horizontal = viewport_width * Vec3(1.0, 0.0, 0.0);
        let vertical = viewport_height * Vec3(0.0, 1.0, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}

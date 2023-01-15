use std::ops::Range;

use crate::{ray::Ray, vec3::Vec3};

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    // outward normal
    pub normal: Vec3,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<Hit>;
}

impl Object for [Box<dyn Object>] {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<Hit> {
        self.iter()
            .filter_map(|object| object.hit(ray, t_range.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let desciminant = b * b - a * c;
        if desciminant > 0.0 {
            for &t in &[(-b - desciminant.sqrt()) / a, (-b + desciminant.sqrt()) / a] {
                if t_range.contains(&t) {
                    return Some(Hit {
                        t,
                        p: ray.point_at(t),
                        normal: (ray.point_at(t) - self.center) / self.radius,
                    });
                }
            }
        }
        None
    }
}

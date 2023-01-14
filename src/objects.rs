use crate::{ray::Ray, vec3::Vec3};

pub trait Object {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Hit {}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let desciminant = b * b - a * c;
        if desciminant > 0.0 {
            Some(Hit {})
        } else {
            None
        }
    }
}

use crate::tracing::{ray::Ray, vec3::Point3};

use super::{HitRecord, Hittable};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn empty() -> Sphere {
        Sphere {
            center: Point3::empty(),
            radius: 0.0,
        }
    }
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_bounds.0 || root > t_bounds.1 {
                root = (-half_b + sqrtd) / a;
                if root < t_bounds.0 || root > t_bounds.1 {
                    None
                } else {
                    let p = ray.at(root);
                    let hr = HitRecord::new(p, root, (p - &self.center) / self.radius, ray);
                    Some(hr)
                }
            } else {
                let p = ray.at(root);
                let hr = HitRecord::new(p, root, (p - &self.center) / self.radius, ray);
                Some(hr)
            }
        }
    }
}

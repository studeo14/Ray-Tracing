use std::sync::Arc;

use crate::{Anchored, Scatter, tracing::{ray::Ray, vec3::Point3}};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
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
                    let hr = HitRecord::new(
                        p,
                        root,
                        (p - &self.center) / self.radius,
                        ray,
                        self.mat.clone(),
                    );
                    Some(hr)
                }
            } else {
                let p = ray.at(root);
                let hr = HitRecord::new(
                    p,
                    root,
                    (p - &self.center) / self.radius,
                    ray,
                    self.mat.clone(),
                );
                Some(hr)
            }
        }
    }
}

impl Anchored for Sphere {
    fn origin(&self) -> Point3 {
        self.center
    }

    fn set_origin(&mut self, origin: Point3) {
        self.center = origin;
    }
}


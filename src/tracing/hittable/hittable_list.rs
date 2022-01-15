use crate::tracing::ray::Ray;

use super::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn from(object: Box<dyn Hittable + Send + Sync>) -> HittableList {
        let mut h = HittableList::new();
        h.add(object);
        h
    }
    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord> {
        let mut ret_val = None;
        let mut closest_so_far = t_bounds.1;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, (t_bounds.0, closest_so_far)) {
                closest_so_far = rec.t;
                ret_val = Some(rec);
            }
        }
        ret_val
    }
}

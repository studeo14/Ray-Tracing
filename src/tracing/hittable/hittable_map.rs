use std::collections::HashMap;

use crate::{tracing::ray::Ray, Anchored, HitRecord, Hittable};

pub struct HittableMap {
    pub objects: HashMap<String, Box<dyn Anchored>>,
}

impl HittableMap {
    pub fn new() -> HittableMap {
        HittableMap {
            objects: HashMap::new(),
        }
    }
    pub fn from(name: String, object: Box<dyn Anchored>) -> HittableMap {
        let mut h = HittableMap::new();
        h.add(name, object);
        h
    }
    pub fn add(&mut self, name: String, object: Box<dyn Anchored>) {
        self.objects.insert(name, object);
    }
}

impl Hittable for HittableMap {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord> {
        let mut ret_val = None;
        let mut closest_so_far = t_bounds.1;

        for (_name, object) in &self.objects {
            if let Some(rec) = object.hit(ray, (t_bounds.0, closest_so_far)) {
                closest_so_far = rec.t;
                ret_val = Some(rec);
            }
        }
        ret_val
    }
}

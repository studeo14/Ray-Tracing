use std::sync::Arc;

use crate::{
    tracing::{Point3, Ray, Vec3},
    Scatter,
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Scatter + Send + Sync>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: f64,
        outward_normal: Vec3,
        ray: &Ray,
        mat: Arc<dyn Scatter + Send + Sync>,
    ) -> HitRecord {
        if ray.direction().dot(&outward_normal) > 0.0 {
            HitRecord {
                p,
                t,
                normal: -outward_normal,
                front_face: false,
                mat,
            }
        } else {
            HitRecord {
                p,
                t,
                normal: outward_normal,
                front_face: true,
                mat,
            }
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

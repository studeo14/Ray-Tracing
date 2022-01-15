use crate::tracing::{Point3, Ray, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3::empty(),
            normal: Vec3::empty(),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn new(p: Point3, t: f64, outward_normal: Vec3, ray: &Ray) -> HitRecord {
        if ray.direction().dot(&outward_normal) > 0.0 {
            HitRecord {
                p,
                t,
                normal: -outward_normal,
                front_face: false,
            }
        } else {
            HitRecord {
                p,
                t,
                normal: outward_normal,
                front_face: true,
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

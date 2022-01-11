use crate::{ray::Ray, vec3::{Point3, Vec3}};

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
            front_face: false
        }
    }
    pub fn new(p: Point3, t: f64, outward_normal: Vec3, ray: &Ray) -> HitRecord {
        if ray.direction().dot(&outward_normal) < 0.0 {
            HitRecord {
                p,
                t,
                normal: outward_normal,
                front_face: true,
            }
        } else {
            HitRecord {
                p,
                t,
                normal: -outward_normal,
                front_face: false,
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord>;
}

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
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
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
                    let hr = HitRecord::new(p, root, p - &self.center / self.radius, ray);
                    Some(hr)
                }
            } else {
                let p = ray.at(root);
                let hr = HitRecord::new(p, root, p - &self.center / self.radius, ray);
                Some(hr)
            }
        }
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{objects: Vec::new()}
    }
    pub fn from(object: Box<dyn Hittable>) -> HittableList {
        let mut h = HittableList::new();
        h.add(object);
        h
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
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

pub mod sphere;
pub use sphere::*;
pub mod hittable_list;
pub use hittable_list::*;
pub mod hittable_map;
pub use hittable_map::*;
pub mod hit_record;
pub use hit_record::*;
pub mod scatter;
pub use scatter::*;
pub mod materials;

use crate::{Point3, Ray};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord>;
}

pub trait Anchored: Hittable {
    fn origin(&self) -> Point3;
    fn set_origin(&mut self, origin: Point3);
}

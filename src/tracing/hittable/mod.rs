pub mod sphere;
pub use sphere::*;
pub mod hittable_list;
pub use hittable_list::*;
pub mod hit_record;
pub use hit_record::*;
pub mod scatter;
pub use scatter::*;
pub mod materials;

use crate::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_bounds: (f64, f64)) -> Option<HitRecord>;
}

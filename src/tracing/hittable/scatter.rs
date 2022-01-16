use crate::{Color, HitRecord, Ray};

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

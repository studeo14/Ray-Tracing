use crate::{Color, HitRecord, Ray, Scatter, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {albedo}
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut direction = hit_record.normal + Vec3::random_unit_vector();
        if direction.near_zero() {
            direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.p, direction);

        Some((self.albedo, scattered))
    }
}

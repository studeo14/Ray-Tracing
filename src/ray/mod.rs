use crate::{color::Color, hittable::Hittable, vec3::{Point3, Vec3}};


pub struct Ray {
    m_orig: Point3,
    m_dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            m_orig: origin,
            m_dir: direction
        }
    }
    pub fn empty() -> Ray {
        Ray {
            m_orig: Point3::empty(),
            m_dir: Vec3::empty()
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.m_orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.m_dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.m_orig + (t*self.m_dir)
    }
    pub fn ray_color(&self, world: &impl Hittable) -> Color {
            let unit_direction = self.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

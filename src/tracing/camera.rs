use crate::{Point3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    m_origin: Point3,
    m_lower_left_corner: Point3,
    m_horizontal: Vec3,
    m_vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::empty();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            m_origin: origin,
            m_horizontal: horizontal,
            m_vertical: vertical,
            m_lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.m_origin,
            self.m_lower_left_corner + u * self.m_horizontal + v * self.m_vertical - self.m_origin,
        )
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

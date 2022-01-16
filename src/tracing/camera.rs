use crate::{Point3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    m_origin: Point3,
    m_lower_left_corner: Point3,
    m_horizontal: Vec3,
    m_vertical: Vec3,
    m_cu: Vec3,
    m_cv: Vec3,
    m_lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperature: f64,
        focus_dist: f64,
    ) -> Camera {
        // vertical fov
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).unit_vector();
        let cu = vup.cross(&cw).unit_vector();
        let cv = cw.cross(&cu);

        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;

        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            m_origin: lookfrom,
            m_horizontal: h,
            m_vertical: v,
            m_lower_left_corner: llc,
            m_cu: cu,
            m_cv: cv,
            m_lens_radius: aperature / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.m_lens_radius * Vec3::random_in_unit_disk();
        let offset = self.m_cu * rd.x() + self.m_cv * rd.y();

        Ray::new(
            self.m_origin + offset,
            self.m_lower_left_corner + u * self.m_horizontal + v * self.m_vertical
                - self.m_origin
                - offset,
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

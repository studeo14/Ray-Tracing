use image::Rgb;

use crate::{camera, Vec3};

pub type Color = Vec3;

pub trait ConvertToRGB {
    fn to_rgb(&self) -> Rgb<u8>;
    fn to_rgb_aa(&self, samples: u32) -> Rgb<u8>;
}

impl ConvertToRGB for Color {
    fn to_rgb(&self) -> Rgb<u8> {
        Rgb([
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8,
        ])
    }

    fn to_rgb_aa(&self, samples: u32) -> Rgb<u8> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        Rgb([
            (256.0 * camera::clamp(r, 0.0, 0.999)) as u8,
            (256.0 * camera::clamp(g, 0.0, 0.999)) as u8,
            (256.0 * camera::clamp(b, 0.0, 0.999)) as u8,
        ])
    }
}

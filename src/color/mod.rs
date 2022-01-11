use image::Rgb;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub trait ConvertToRGB {
    fn to_rgb(&self) -> Rgb<u8>;
}

impl ConvertToRGB for Color {
    fn to_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8,
        ])
    }
}

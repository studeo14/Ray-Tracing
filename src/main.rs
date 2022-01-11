use std::io::Write;

use ray_tracing::{color::ConvertToRGB, hittable::{HittableList, Sphere}, ray::Ray, vec3::{Point3, Vec3}};

/*
 * Ray Tracting Example
 */

fn main() {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::empty();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let mut line: Option<u32> = None;

    print!("P6\n{} {} 255\n", image_width, image_height);

    let mut stderr = std::io::stderr();

    for j in (0..image_height).rev() {
        if line != Some(j) {
            write!(stderr, "{}[1K\rScanlines: remaining: {}", 27 as char, j).expect("Unable to write");
            stderr.flush().expect("Unable to flush");
            line = Some(j);
        }
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = (j) as f64 / (image_height - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color = ray.ray_color(&world);
            println!("{}", color);
        }
    }

}

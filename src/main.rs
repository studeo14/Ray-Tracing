use std::{io::Write, sync::{Arc, mpsc}};

use image::RgbImage;
use ray_tracing::{Camera, Color, ConvertToRGB, HittableList, Point3, Sphere, materials::{Dielectric, Lambertian, Metal}};
use threadpool::ThreadPool;

/*
 * Ray Tracting Example
 */

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 720;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 256;
    let max_depth = 512;

    // World
    let mut world = HittableList::new();
    // - materials
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_left_inner = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    // - objects
    let sphere_groud = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, mat_left_inner);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    world.add(Box::new(sphere_groud));
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_left_inner));
    world.add(Box::new(sphere_right));
    let world_arc = Arc::new(world);

    // Camera
    let camera = Camera::new();
    let camera_arc = Arc::new(camera);

    // Render
    // -- TP
    let threadpool = ThreadPool::new(20);
    let mut img = RgbImage::new(image_width, image_height);

    let (res_tx, res_rx) = mpsc::channel();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let res_tx = res_tx.clone();
            let world_arc = Arc::clone(&world_arc);
            let camera_arc = Arc::clone(&camera_arc);
            threadpool.execute(move ||{
                let mut color = Color::empty();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                    let ray = camera_arc.get_ray(u, v);
                    color += ray.ray_color(&world_arc, max_depth);
                }
                res_tx.send((i, image_height - 1 - j, color)).expect("unable to send");
            });
        }
    }
    drop(res_tx);

    let mut pixels_done = 0;
    let mut stdout = std::io::stdout();
    for (x, y, color) in res_rx {
        pixels_done += 1;
        img.put_pixel(x, y, color.to_rgb_aa(samples_per_pixel));
        print!("{}[1K\rPixels Done: {}/{}", 27 as char, pixels_done, image_height * image_width);
        stdout.flush().expect("Unable to flush");
    }
    img.save("test.png").expect("Unable to save image");
    println!("Done");
}

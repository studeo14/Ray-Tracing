use std::{
    io::Write,
    sync::{mpsc, Arc},
};

use image::RgbImage;
use ray_tracing::{random_scene, Camera, Color, ConvertToRGB, Point3, Vec3};
use threadpool::ThreadPool;

/*
 * Ray Tracting Example
 */

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 512;
    let max_depth = 50;

    // World
    let world = random_scene();
    let world_arc = Arc::new(world);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperature,
        dist_to_focus,
    );
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
            threadpool.execute(move || {
                let mut color = Color::empty();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                    let ray = camera_arc.get_ray(u, v);
                    color += ray.ray_color(&world_arc, max_depth);
                }
                res_tx
                    .send((i, image_height - 1 - j, color))
                    .expect("unable to send");
            });
        }
    }
    drop(res_tx);

    let mut pixels_done = 0;
    let mut stdout = std::io::stdout();
    for (x, y, color) in res_rx {
        pixels_done += 1;
        img.put_pixel(x, y, color.to_rgb_aa(samples_per_pixel));
        print!(
            "{}[1K\rPixels Done: {}/{}",
            27 as char,
            pixels_done,
            image_height * image_width
        );
        stdout.flush().expect("Unable to flush");
    }
    img.save("test.png").expect("Unable to save image");
    println!("Done");
}

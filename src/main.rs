use std::sync::Arc;

use ray_tracing::{AnimatedConfig, Animation, CameraTransformer, Point3, SceneConfig, Vec3, animate_scene, random_scene, render_scene};

/*
 * Ray Tracting Example
 */

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;


    // World
    let world = random_scene();
    let world_arc = Arc::new(world);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookfrom_b = Point3::new(16.0, 1.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let num_threads = 16;
    let scene_config = SceneConfig::new(
        ASPECT_RATIO,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        lookfrom,
        lookat,
        vup,
        vfov,
        dist_to_focus,
        aperature,
        num_threads,
    );
    let scene_config_arc = Arc::new(scene_config);

    let camera_mover = CameraTransformer::new(lookfrom, lookfrom_b, 240);
    let scene_transformer = AnimatedConfig {
        config: scene_config_arc,
        transformers: vec![Box::new(camera_mover)]
    };
    let animation = Animation {
        m_world: world_arc,
        m_config: scene_transformer,
    };

    animate_scene("images/scene_a", animation);

}

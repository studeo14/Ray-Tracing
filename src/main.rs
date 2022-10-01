//! Author: Steven Frederiksen
use std::sync::Arc;

use ray_tracing::{random_scene, animate_scene};
use ray_tracing::{
    AnimatedConfig, AnimatedWorld, Animation, CameraTransformer, ObjectMover, Point3, SceneConfig,
    Vec3,
};

/*
 * Ray Tracting Example
 */

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 300;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 30;
    let max_depth = 20;

    // World
    let world = random_scene();
    let world_arc = Arc::new(world);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
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

    // Animation
    let main1_start = Point3::new(0.0, 1.0, 0.0);
    let main1_stop = Point3::new(0.0, 3.0, 0.0);
    let main1_mover = ObjectMover::new("main1".to_string(), main1_start, main1_stop, 240);
    let world_animation = AnimatedWorld {
        world: world_arc,
        transformers: vec![Box::new(main1_mover)],
    };
    let lookfrom_b = Point3::new(10.0, 4.0, 5.0);
    let camera_mover = CameraTransformer::new(lookfrom, lookfrom_b, 240);
    let scene_transformer = AnimatedConfig {
        config: scene_config_arc,
        transformers: vec![Box::new(camera_mover)],
    };
    let animation = Animation {
        m_world: world_animation,
        m_config: scene_transformer,
    };

    if let Err(e) = animate_scene("test_scene.png", animation) {
        println!("{}", e);
    } else {
        println!("Done processing!");
    }
}

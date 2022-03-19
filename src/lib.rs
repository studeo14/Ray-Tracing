mod tracing;
use std::{io::Write, sync::{Arc, mpsc}};
use std::time::Instant;

use image::RgbImage;
use rand::Rng;
use threadpool::ThreadPool;
use tracing::materials::{Dielectric, Lambertian, Metal};
pub use tracing::*;

pub fn random_scene() -> HittableMap {
    let mut rng = rand::thread_rng();
    let mut world = HittableMap::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.add("ground".to_string(), Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.add(format!("random_{}_{}", a, b), Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.add(format!("random_{}_{}", a, b), Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.add(format!("random_{}_{}", a, b), Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add("main1".to_string(), Box::new(sphere1));
    world.add("main2".to_string(), Box::new(sphere2));
    world.add("main3".to_string(), Box::new(sphere3));

    world
}

#[derive(Debug, Copy, Clone)]
pub struct SceneConfig {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub vfov: f64,
    pub dist_to_focus: f64,
    pub aperature: f64,
    pub num_threads: usize,
}

impl SceneConfig {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        image_height: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        dist_to_focus: f64,
        aperature: f64,
        num_threads: usize,
    ) -> SceneConfig {
        SceneConfig {
            aspect_ratio,
            image_height,
            image_width,
            samples_per_pixel,
            max_depth,
            lookfrom,
            lookat,
            vup,
            vfov,
            dist_to_focus,
            aperature,
            num_threads,
        }
    }
}

pub fn render_scene(output_file: &str, world: Arc<impl Hittable + 'static>, scene_config: Arc<SceneConfig>) {
    let camera = Camera::from(&scene_config);
    let camera_arc = Arc::new(camera);
    // Render
    // -- TP
    let threadpool = ThreadPool::new(scene_config.num_threads);
    let mut img = RgbImage::new(scene_config.image_width, scene_config.image_height);

    let (res_tx, res_rx) = mpsc::channel();
    for j in (0..scene_config.image_height).rev() {
        for i in 0..scene_config.image_width {
            let res_tx = res_tx.clone();
            let world_arc = Arc::clone(&world);
            let camera_arc = Arc::clone(&camera_arc);
            let scene_config = Arc::clone(&scene_config);
            threadpool.execute(move || {
                let mut color = Color::empty();
                for _ in 0..scene_config.samples_per_pixel {
                    let u = (i as f64 + rand::random::<f64>()) / (scene_config.image_width - 1) as f64;
                    let v = (j as f64 + rand::random::<f64>()) / (scene_config.image_height - 1) as f64;
                    let ray = camera_arc.get_ray(u, v);
                    color += ray.ray_color(&world_arc, scene_config.max_depth);
                }
                res_tx
                    .send((i, scene_config.image_height - 1 - j, color))
                    .expect("unable to send");
            });
        }
    }
    drop(res_tx);

    let mut pixels_done = 0;
    let mut stdout = std::io::stdout();
    let now = Instant::now();

    for (x, y, color) in res_rx {
        pixels_done += 1;
        img.put_pixel(x, y, color.to_rgb_aa(scene_config.samples_per_pixel));
        print!(
            "{}[1K\rPixels Done: {}/{}",
            27 as char,
            pixels_done,
            scene_config.image_height * scene_config.image_width
        );
        stdout.flush().expect("Unable to flush");
    }
    img.save(output_file).expect("Unable to save image");
    println!("\nDone. Took {:.2?}s", now.elapsed());
}

pub fn animate_scene(output_file_base: &str, animation: Animation) {
    for (ix, (world_arc, scene_config)) in animation.enumerate() {
        let output_file_ix = format!("{}_{:05}.png", output_file_base, ix);
        render_scene(output_file_ix.as_str(), world_arc, scene_config);
        println!("{} done", output_file_ix);
    }
}

pub struct Animation {
    pub m_world: AnimatedWorld,
    pub m_config: AnimatedConfig,
}

impl Iterator for Animation {
    type Item = (Arc<HittableMap>, Arc::<SceneConfig>);

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(world), Some(cfg)) = (self.m_world.next(), self.m_config.next()) {
            Some((world, cfg))
        } else {
            None
        }
    }
}

pub struct AnimatedWorld {
    pub world: Arc<HittableMap>,
    pub transformers: Vec<Box<dyn WorldTransformer>>,
}

impl Iterator for AnimatedWorld {
    type Item = Arc<HittableMap>;

    fn next(&mut self) -> Option<Self::Item> {
        for transformer in &mut self.transformers[..] {
            if !transformer.transform(Arc::get_mut(&mut self.world).unwrap()) {
                return None;
            }
        }
        Some(Arc::clone(&self.world))
    }
}

pub struct AnimatedConfig {
    pub config: Arc<SceneConfig>,
    pub transformers: Vec<Box<dyn SceneTransformer>>,
}

impl Iterator for AnimatedConfig {
    type Item = Arc<SceneConfig>;

    fn next(&mut self) -> Option<Self::Item> {
        for transformer in &mut self.transformers[..] {
            if let Some(cfg) = transformer.transform(&self.config) {
                self.config = Arc::new(cfg);
            } else {
                return None;
            }
        }
        Some(Arc::clone(&self.config))
    }
}

pub trait WorldTransformer {
    fn transform(&mut self, world: &mut HittableMap) -> bool;
}

pub struct ObjectMover {
    t: usize,
    steps: usize,
    step: Point3,
    name: String,
}

impl ObjectMover {
    pub fn new(name: String, starting_point: Point3, stopping_point: Point3, steps: usize) -> ObjectMover {
        let step = (stopping_point - starting_point) / steps as f64;
        ObjectMover {
            name,
            steps,
            t: 0,
            step
        }
    }
}

impl WorldTransformer for ObjectMover {
    fn transform(&mut self, world: &mut HittableMap) -> bool {
        if self.t >= self.steps {
            false
        } else {
            self.t += 1;
            if let Some(object) = world.objects.get_mut(&self.name) {
                object.set_origin(object.origin() + self.step);
                true
            } else {
                false
            }
        }
    }
}

pub trait SceneTransformer {
    fn transform(&mut self, scene_config: &SceneConfig) -> Option<SceneConfig>;
}

pub struct CameraTransformer {
    t: usize,
    steps: usize,
    step: Point3,
}

impl CameraTransformer {
    pub fn new(starting_point: Point3, stopping_point: Point3, steps: usize) -> CameraTransformer {
        let step = (stopping_point - starting_point) / steps as f64;
        CameraTransformer {
            steps,
            t: 0,
            step
        }
    }
}

impl SceneTransformer for CameraTransformer {
    fn transform(&mut self, scene_config: &SceneConfig) -> Option<SceneConfig> {
        if self.t >= self.steps {
            None
        } else {
            self.t += 1;
            let mut new_scene = scene_config.clone();
            new_scene.lookfrom += self.step;
            Some(new_scene)
        }
    }
}

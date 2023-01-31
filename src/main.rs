mod camera;
mod color;
mod image;
mod material;
mod objects;
mod point3d;
mod ray;
mod raytracer;
mod config;

use camera::Camera;
use color::*;
use image::*;
use material::*;
use objects::{Hittable, Sphere};
use point3d::*;
use raytracer::*;
use config::*;


fn main() {
    let c = Config::new();

    let mut ray_tracer = RayTracer::new(c.image, c.camera, c.objects, c.light, c.depth);
    ray_tracer.render();
}

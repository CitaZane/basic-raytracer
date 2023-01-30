mod point3d;
mod color;
mod ray;
mod objects;
mod camera;
mod raytracer;
mod image;
mod material;

use camera::Camera;
use material::*;
use point3d::*;
use color::*;
use objects::{sphere::*, Hittable, Cube};
use raytracer::*;
use image::*;
use std::f64::consts::PI;

use crate::objects::Plane;

fn main() {
    let width = 300;
    let height = 200;
    // Image
    let image = Image::new(width, height);
    // Camera
    let origin = Point3D::new(0.,2., 0.);
    let target = Point3D::new(0.,0.,-4.);
    let up = Point3D::new(0.,1.,0.);
    // let fov= 35.;
    let fov = (25.0 * PI) / 180.0;
    let aspect_ratio = width as f64 / height as f64;
    let camera = Camera::new(origin, target,up, fov, aspect_ratio);

    // objects
    let center = Point3D::new(0., 0., -6.);
    let matte = Matte::new(Color::green());
    let material = Material::Matte(matte);
    let sphere = Sphere::new(center, 1., material);
    let object:Box<dyn Hittable>= Box::new(sphere);

    let mut objects: Vec<Box<dyn Hittable>> = vec![object];

    let plane_point = Point3D::new(0.,-1.,0.);
    let plane_n = Point3D::new(0., 8., 0.);
    let plane = Plane::new(plane_point, plane_n.unit_vector());
    let object_2:Box<dyn Hittable>= Box::new(plane);
    objects.push(object_2);

    let center_3= Point3D::new(2., -0.5, -6.);
    let metal = Metal::new();
    let material = Material::Metal(metal);
    let sphere_3 = Sphere::new(center_3, 0.5, material);
    let object_3:Box<dyn Hittable>= Box::new(sphere_3);
    objects.push(object_3);

    // let center_4= Point3D::new(0., -151., -6.);
    // let matte = Matte::new(Color::new(0.7,0.7,0.7));
    // let material = Material::Matte(matte);
    // let sphere_4 = Sphere::new(center_4, 150., material);
    // let object_4:Box<dyn Hittable>= Box::new(sphere_4);
    // objects.push(object_4);

    // let center_5= Point3D::new(-2., -0.5, -4.);
    // let metal = Metal::new();
    // let material = Material::Metal(metal);
    // let sphere_5 = Sphere::new(center_5, 0.5, material);
    // let object_5:Box<dyn Hittable>= Box::new(sphere_5);
    // objects.push(object_5);

    let min = Point3D::new(-2., -1., -4.);
    let max = Point3D::new(-1., 0., -5.);
    let matte = Matte::new(Color::red());
    let material = Material::Matte(matte);
    let cube = Cube::new(min,max, material);
    let object_5:Box<dyn Hittable>= Box::new(cube);
    objects.push(object_5);

    // LIGHT
    let light_center= Point3D::new(5., 10., -5.);
    let l = Light::new(0.5);
    let material = Material::Light(l);
    let light = Sphere::new(light_center, 1., material);

    let mut ray_tracer = RayTracer::new(image, camera, objects, light);
    ray_tracer.render();

}

// to run program
// cargo run > output.ppm
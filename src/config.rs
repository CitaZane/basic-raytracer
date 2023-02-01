use crate::{
    camera::Camera,
    color::Color,
    image::Image,
    material::{Material, Matte},
    objects::{Hittable, Sphere, Cube, Plane, Cylinder},
    point3d::Point3D,
};
use std::f64::consts::PI;

pub struct Config {
    pub image: Image,
    pub camera: Camera,
    pub light: Sphere,
    pub objects: Vec<Box<dyn Hittable>>,
    pub depth: i32,
}

impl Config {
    pub fn new() -> Self {
        // Image
        let width = 300;
        let height = 200;
        let samples_per_pixel = 25;
        let depth = 50;

        // Camera
        let origin = Point3D::new(0., 1., 0.);
        let direction = Point3D::new(0., 0., -4.);
        let up = Point3D::new(0., 1., 0.);
        let fov = 25.;
        
        // Light
        let center = Point3D::new(5., 10., -5.);
        let intensity = 0.5;
        
        // scene
        let objects = Config::scene_three();
        
        // helpers
        let aspect_ratio = width as f64 / height as f64;
        let fov_calc = (fov * PI) / 180.0;

        // Return Config object
        Self {
            image: Image::new(width, height, samples_per_pixel),
            camera: Camera::new(origin, direction, up, fov_calc, aspect_ratio),
            light: Sphere::new_light(center, intensity),
            objects,
            depth
        }
    }
    #[allow(dead_code)]
    fn scene_one() -> Vec<Box<dyn Hittable>> {
        let mut objects: Vec<Box<dyn Hittable>> = vec![];

        let center = Point3D::new(0., 0., -6.);
        let matte = Matte::new(Color::green());
        let material = Material::Matte(matte);
        let sphere = Sphere::new(center, 1., material);
        let object: Box<dyn Hittable> = Box::new(sphere);

        objects.push(object);
        objects
    }
    #[allow(dead_code)]
    fn scene_two() -> Vec<Box<dyn Hittable>> {
        let mut objects: Vec<Box<dyn Hittable>> = vec![];
        // cube
        let min = Point3D::new(-2., -1., -4.);
        let max = Point3D::new(-1., 0., -5.);
        let matte = Matte::new(Color::red());
        let material = Material::Matte(matte);
        let cube = Cube::new(min, max, material);
        let cube_obj: Box<dyn Hittable> = Box::new(cube);
        objects.push(cube_obj);

        // plane
        let plane_point = Point3D::new(0., -1., 0.);
        let plane_n = Point3D::new(0., 8., 0.);
        let plane = Plane::new(plane_point, plane_n.unit_vector());
        let plane_obj: Box<dyn Hittable> = Box::new(plane);
        objects.push(plane_obj);

        objects
    }
    #[allow(dead_code)]
    fn scene_three() -> Vec<Box<dyn Hittable>> {
        let mut objects: Vec<Box<dyn Hittable>> = vec![];
        // cube
        let min = Point3D::new(-2., -1., -4.);
        let max = Point3D::new(-1., 0., -5.);
        let material = Material::matte(Color::red());
        let cube = Cube::new(min, max, material);
        let cube_obj: Box<dyn Hittable> = Box::new(cube);
        objects.push(cube_obj);

        // sphere
        let center = Point3D::new(0., 0., -6.);
        // let material = Material::matte(Color::green());
        let radius = 1.;
        // alternative material option
        let material = Material::metal();
        let sphere = Sphere::new(center,radius, material);
        let sphere_obj: Box<dyn Hittable> = Box::new(sphere);
        objects.push(sphere_obj);

        // plane
        let plane_point = Point3D::new(0., -1., 0.);
        let plane_normal = Point3D::new(0., 8., 0.).unit_vector();
        let plane = Plane::new(plane_point, plane_normal);
        let plane_obj: Box<dyn Hittable> = Box::new(plane);
        objects.push(plane_obj);

        // cylinder
        let base = Point3D::new(2., -1. ,-7.);
        let r = 0.5;
        let height = 2.;
        let material = Material::matte(Color::green());
        let cylinder = Cylinder::new(base,r,  material, height);
        let cylinder_obj : Box<dyn Hittable> = Box::new(cylinder);
        objects.push(cylinder_obj);

        objects
    }
}

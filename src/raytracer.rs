use crate::*;
use rand::prelude::*;

pub struct RayTracer {
    max_depth: i32,

    pub camera: Camera,
    pub objects: Vec<Box<dyn Hittable>>,
    pub image: Image,
    pub light: Sphere,
}

impl RayTracer {
    pub fn new(
        image: Image,
        camera: Camera,
        objects: Vec<Box<dyn Hittable>>,
        light: Sphere,
    ) -> Self {
        Self {
            image,
            camera,
            objects,
            light,
            max_depth:50,
        }
    }

    pub fn render(&mut self) {
        for x in 0..self.image.width {
            eprintln!("Line: {x} of {}", self.image.width);
            for y in 0..self.image.height {
                let pixel = self.intersect(x, y);
                self.image.set_pixel(x, y, pixel);
            }
        }
        eprintln!("Done");
        self.image.save_image();
    }
    fn intersect(&self, col: usize, row: usize) -> Color {
        let mut pixel = Color::new(0., 0., 0.);
        for _i in 0..self.image.samples_per_pixel {
            let u = (2.0 * (col as f64 + random_float())) / self.image.width as f64 - 1.0;
            let v = (-2.0 * (row as f64) + random_float()) / self.image.height as f64 + 1.0;
            let ray = self.camera.make_ray(u, v);
            let sample_pixel = self.find_pixel_color(&ray, self.max_depth);
            pixel = pixel + sample_pixel;
        }
        pixel
    }

    fn find_pixel_color(&self, ray: &ray::Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::black();
        }
        let intersection = self.hit_scene(&ray);

        if intersection.is_some() {
            let intersecion = intersection.unwrap();
            let bounce = match intersecion.material {
                Material::Matte(m) => m.scatter(&ray, &intersecion),
                Material::Metal(m) => m.scatter(&ray, &intersecion),
                Material::Light(l) => l.scatter(&ray, &intersecion),
            };
            if bounce.is_none() {
                return Color::black();
            }
            let (bounce_ray, color) = bounce.unwrap();
            let target_color = self.find_pixel_color(&bounce_ray, depth - 1);
            let pixel_color = color * target_color * 0.5;
            // Check if light source direct
            let light_blocked = self.cast_ray_to_light(&intersecion.point);
            if light_blocked.is_some() {
                return pixel_color;
            }
            return self.light_specular_diffuse_adjustment(&intersecion) + pixel_color;
        } else {
            // return sky
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Color::white() * (1. - t) + Color::new(0.5, 0.7, 1.) * t;
            // return Color::black();
        }
    }
    fn hit_scene(&self, ray: &ray::Ray) -> Option<objects::Intersection> {
        let mut closest_intersection = None;
        let mut closest_hit = f64::MAX;

        self.objects.iter().for_each(|object| {
            let hit_record = object.hit(&ray, 0.001, closest_hit);
            if hit_record.is_some() {
                let new_record = hit_record.unwrap();
                if new_record.t < closest_hit {
                    closest_hit = new_record.t;
                    closest_intersection = Some(new_record);
                }
            }
        });
        closest_intersection
    }
    fn cast_ray_to_light(&self, intersection_point: &Point3D) -> Option<objects::Intersection> {
        let light_vec = (self.light.center - *intersection_point).unit_vector();
        let shadow_ray = ray::Ray::new(intersection_point.clone(), light_vec);
        self.hit_scene(&shadow_ray)
    }
    fn light_specular_diffuse_adjustment(&self, intersection: &objects::Intersection) -> Color {
        const DIFFUSE_COEF: f64 = 0.4;
        const SPECULAR_COEF: f64 = 0.2;

        let light_vec = (self.light.center - intersection.point).unit_vector();
        let light_angle = light_vec.dot(&intersection.normal);
        let reflection_vec =
            light_vec * -2. * light_vec.dot(&intersection.normal) * intersection.normal;
        let view_direction = (intersection.point - self.camera.origin).unit_vector();
        let reflection_angle = reflection_vec.dot(&view_direction);

        let diffuse_reflection = if light_angle < 0. { 0. } else { light_angle };
        let specular = if reflection_angle < 0. {
            0.
        } else {
            reflection_angle
        };
        let specular_reflection = specular.powf(1.);

        (Color::white() * DIFFUSE_COEF * diffuse_reflection
            + Color::white() * SPECULAR_COEF * specular_reflection)
            * self.light.material.intensity()
    }
}

// in range 0 -1
fn random_float() -> f64 {
    rand::thread_rng().gen()
}

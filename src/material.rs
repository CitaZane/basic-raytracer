use crate::{color::Color, objects::Intersection, point3d::Point3D, ray::Ray};

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &Intersection) -> Option<(Ray, Color)>;
}

pub enum Material {
    Matte(Matte),
    Metal(Metal),
    Light(Light),
}
impl Material {
    pub fn intensity(&self) -> f64 {
        match self {
            Material::Matte(_) => 1.,
            Material::Metal(_) => 1.,
            Material::Light(l) => l.intensity,
        }
    }
}
impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Ray, Color)> {
        match self {
            Material::Matte(m) => m.scatter(ray, intersection),
            Material::Metal(m) => m.scatter(ray, intersection),
            Material::Light(l) => l.scatter(ray, intersection),
        }
    }
}

pub struct Metal {
    color: Color,
}

impl Metal {
    pub fn new() -> Self {
        let color = Color::white();
        Self { color }
    }
    fn reflect(v: &Point3D, n: &Point3D) -> Point3D {
        *v - *n * 2. * v.dot(n)
    }
}
impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Ray, Color)> {
        let reflected = Metal::reflect(&ray.direction.unit_vector(), &intersection.normal);
        let scattered = Ray::new(intersection.point, reflected);
        if scattered.direction.dot(&intersection.normal) < 0.0 {
            return None;
        }
        Some((scattered, self.color.clone()))
    }
}
pub struct Matte {
    color: Color,
}

impl Matte {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Scatterable for Matte {
    fn scatter(&self, _ray: &Ray, intersection: &Intersection) -> Option<(Ray, Color)> {
        let mut scatter_direction = intersection.normal + Point3D::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = intersection.normal;
        }
        let target = intersection.point + scatter_direction;
        let bounced_ray = Ray::new(intersection.point, target - intersection.point);
        Some((bounced_ray, self.color.clone()))
    }
}

pub struct Light {
    intensity: f64,
}
impl Light {
    pub fn new(intensity: f64) -> Self {
        Self { intensity }
    }
}

impl Scatterable for Light {
    fn scatter(&self, _ray: &Ray, _intersection: &Intersection) -> Option<(Ray, Color)> {
        None
    }
}

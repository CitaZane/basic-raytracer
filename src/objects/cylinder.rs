use crate::material::Material;
use crate::point3d::Point3D;
use crate::ray::Ray;

use super::{Hittable, Intersection};
pub struct Cylinder {
    pub material: Material,
    pub base: Point3D,
    axis: Point3D,
    pub radius: f64,
    pub height: f64,
}

impl Cylinder {
    pub fn new(base: Point3D, radius: f64, material: Material, height: f64) -> Self {
        Self {
            base,
            axis: Point3D::new(0., 1.,0.),
            radius,
            material,
            height,
        }
    }
    fn check_cap(
        &self,
        ray: &Ray,
        normal: Point3D,
        t_min: f64,
        t_max: f64,
    ) -> Option<(f64, Point3D, Point3D)> {
        let denom = ray.direction.dot(&normal);
        if denom > t_min || denom < -1. * t_min {
            let center = self.base + Point3D::new(0., self.height, 0.);
            let t = (center - ray.origin).dot(&normal) / denom;
            if t >= 0.0 && t < t_max {
                let point = ray.at(t)* 1.001;
                if (point.x() - self.base.x()).powf(2.0) + (point.z() - self.base.z()).powf(2.0)
                    < self.radius.powf(2.0)
                {
                    return Some(( t, normal, point));
                }
            }
        }
        None
    }
    fn intersect_caps(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(f64, Point3D, Point3D)> {
        let mut normal = Point3D::new(0., 1., 0.);
        // top
        let intersect = self.check_cap(ray, normal, t_min, t_max);
        if intersect.is_some() {
            return Some(intersect.unwrap());
        }
        // bottom
        normal.set_y(-1.);
        let intersect = self.check_cap(ray, normal, t_min, t_max);
        if intersect.is_some() {
            return Some(intersect.unwrap());
        }
        None
    }
    fn intersect_body(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option< (f64, Point3D, Point3D)> {
        let oc = ray.origin - self.base;
        let a = (ray.direction.x().powf(2.)) + (ray.direction.z().powf(2.));
        let b = 2. * (oc.x() * ray.direction.x() + oc.z() * ray.direction.z());
        let c = oc.x().powf(2.) + oc.z().powf(2.) - self.radius * self.radius;

        let disc = b.powf(2.) - 4. * a * c;
        if disc < 0.0 {
            return None;
        }
        let sqrtd = disc.sqrt();
        let mut t1 = (-b - sqrtd) / (2. * a);
        let t2 = (-b + sqrtd) / (2. * a);
        if t1 > t2 {
            t1 = t2
        };
        if t1 < t_min || t_max < t1 {
            return None;
        }
        let point = ray.at(t1);
        // turncate
        if point.y() < self.base.y() || point.y() > self.base.y() + self.height {
            return None;
        }
        let m = ray.direction.dot(&self.axis) * t1 + oc.dot(&self.axis);
        let normal = (point - self.base - self.axis * m).unit_vector();
        Some((t1, normal, point))
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut normal = Point3D::new(0., 0., 0.);
        let mut point = Point3D::new(0., 0., 0.);
        let mut t = 0.0;

        let cap_intersect = self.intersect_caps(ray, t_min, t_max);
        if cap_intersect.is_some() {
            let ( cap_t, cap_normal, cap_point) = cap_intersect.unwrap();
            point = cap_point;
            normal = cap_normal;
            t= cap_t;
        } else {
            let intersect = self.intersect_body(ray, t_min, t_max);
            if intersect.is_none(){
                return None;
            } else {
                let(body_t, body_normal, body_point) = intersect.unwrap();
                point = body_point;
                normal = body_normal;
                t= body_t;
            }
        }
        Some(Intersection {
            point,
            t,
            normal,
            material: &self.material,
        })
    }
}

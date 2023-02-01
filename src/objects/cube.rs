use super::{Hittable, Intersection};
use crate::{material::*, point3d::Point3D, ray::Ray};

pub struct Cube {
    pub min: Point3D,
    pub max: Point3D,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Point3D, max: Point3D, material: Material) -> Self {
        Self { min, max, material }
    }
    fn normal(&self, point: &Point3D) -> Point3D {
        const EPSI: f64 = 0.01;
        let mut normal = Point3D::new(0., 0., 0.);
        if (point.x() - self.min.x()).abs() < EPSI {
            normal.set_x(-1.)
        };
        if (point.x() - self.max.x()).abs() < EPSI {
            normal.set_x(1.)
        };
        if (point.y() - self.min.y()).abs() < EPSI {
            normal.set_y(-1.)
        };
        if (point.y() - self.max.y()).abs() < EPSI {
            normal.set_y(1.)
        };
        if (point.z() - self.min.z()).abs() < EPSI {
            normal.set_z(-1.)
        };
        if (point.z() - self.min.z()).abs() < EPSI {
            normal.set_z(1.)
        };
        normal
    }
}

impl Hittable for Cube {
    fn hit<'a>(&'a self, ray: &Ray, hit_record: &mut Intersection<'a>) -> bool {
        let mut t_min = (self.min.x() - ray.origin.x()) / ray.direction.x();
        let mut t_max = (self.max.x() - ray.origin.x()) / ray.direction.x();
        if t_min > t_max {
            (t_min, t_max) = (t_max, t_min)
        }

        let mut t_y_min = (self.min.y() - ray.origin.y()) / ray.direction.y();
        let mut t_y_max = (self.max.y() - ray.origin.y()) / ray.direction.y();
        if t_y_min > t_y_max {
            (t_y_min, t_y_max) = (t_y_max, t_y_min)
        }

        if t_min > t_y_max || t_y_min > t_max {
            return false;
        }
        if t_y_min > t_min {
            t_min = t_y_min
        }
        if t_y_max < t_max {
            t_max = t_y_max
        }

        let mut t_z_min = (self.min.z() - ray.origin.z()) / ray.direction.z();
        let mut t_z_max = (self.max.z() - ray.origin.z()) / ray.direction.z();
        if t_z_min > t_z_max {
            (t_z_min, t_z_max) = (t_z_max, t_z_min)
        }

        if t_min > t_z_max || t_z_min > t_max {
            return false;
        }
        if t_z_min > t_min {
            t_min = t_z_min
        }

        if t_min < hit_record.t_min || t_min > hit_record.t{
            return false;
        }
        if t_min > hit_record.t{
            return false
        }
        let point = ray.at(t_min);
        let normal = self.normal(&point);

        hit_record.point = point;
        hit_record.normal = normal;
        hit_record.t = t_min;
        hit_record.material = Some(&self.material);
        hit_record.hit_anything = true;

        true

    }
}

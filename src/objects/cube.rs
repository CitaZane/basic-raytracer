use super::{Hittable, Intersection};
use crate::{material::*, point3d::Point3D, ray::Ray};

pub struct Cube {
    pub min: Point3D,
    pub max: Point3D,
    pub material: Material,
}
// In the example code, min and max are used to define
// the minimum and maximum corner points of the cube.
// min is a point that defines the corner of the cube with the
// smallest x, y and z coordinates, and max is a point that
// defines the corner of the cube with the largest x, y and z coordinates.

// For example, if a cube is defined with the minimum corner
// at (-0.5, -0.5, -0.5) and the maximum corner
// at (0.5, 0.5, 0.5), this cube would have a size of 1
// along each dimension (x, y, z) and its center would be at the point (0,0,0)

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
    fn hit(
        &self,
        ray: &Ray,
        t_min_bound: f64,
        t_max_bound: f64,
        hit_record: &mut Intersection,
    ) -> bool {
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

        if t_min < t_min_bound || t_min > t_max_bound {
            return false;
        }
        let point = ray.at(t_min);
        let normal = self.normal(&point);

        hit_record.point = point;
        hit_record.normal = normal;
        hit_record.t = t_min;
        hit_record.material = Some(self.material);

        true

        // Some(Intersection {
        //     t: t_min,
        //     point,
        //     normal,
        //     material: &self.material,
        // })
    }
}

use crate::{point3d::Point3D};

pub struct Ray {
    pub origin :Point3D,
    pub direction :Point3D,
}

impl Ray{
    pub fn new(origin: Point3D, direction:Point3D)-> Self{
        Self{origin, direction}
    }
    // position along 3D line - linear interpolation
    pub fn at(&self, t:f64) -> Point3D{
        self.origin + self.direction * t
    }

}
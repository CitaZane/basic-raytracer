use crate::{point3d::Point3D, ray::Ray};

#[derive(Debug)]
pub struct Camera {
    pub origin: Point3D,
    pub forward: Point3D,
    pub up: Point3D,
    pub right: Point3D,
    pub width: f64,
    pub height: f64,
}

impl Camera {
    pub fn new(
        origin: Point3D,
        target: Point3D,
        up_guide: Point3D,
        fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let forward = (target - origin).unit_vector();
        let right = forward.cross(&up_guide).unit_vector();
        let up = right.cross(&forward);
        let height = fov.tan();
        let width = height * aspect_ratio;

        Self {
            origin,
            forward,
            up,
            right,
            width,
            height,
        }
    }
    pub fn make_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.forward + self.right * self.width * u + self.up * self.height * v;
        Ray::new(self.origin, direction.unit_vector())
    }
}

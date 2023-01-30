use crate::{point3d::Point3D, ray::Ray, color::Color, material::*};

use super::{Hittable, Intersection};


pub struct Plane{
    point: Point3D,
    normal:Point3D,
    material:Material
}

impl Plane{
    pub fn new(point:Point3D, normal:Point3D)-> Self{
        let matte = Matte::new(Color::gray());
        let material = Material::Matte(matte);
        Self{point,normal, material,}
    }
}

impl Hittable for Plane{
    fn hit(&self, ray:&Ray, t_min:f64, t_max:f64)-> Option<Intersection>{
        let d_dot_n = self.normal.dot(&ray.direction);
        if d_dot_n == 0.0{return None}//in case of ray paralel
        let t = self.normal.dot(&(self.point - ray.origin)) / d_dot_n;
        if t<= t_min || t>= t_max {return None}
        let hit_record = Intersection{
            t,
            point :ray.at(t),
            normal :self.normal,
            front_face:true,
            material: &self.material,
            };
        
        Some(hit_record)
    }
}
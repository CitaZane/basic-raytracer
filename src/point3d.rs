use std::ops::{Add, Mul, Sub,  Div};
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D{
    pub fn new(x: f64, y:f64, z:f64) -> Self{
        Self{x,y,z}
    }
    pub fn random(min:f64, max:f64)-> Self{
        let mut rng = rand::thread_rng();
        Point3D::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }
    pub fn random_in_unit_sphere()->Self{
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() <1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_vector()->Self{
        Point3D::random_in_unit_sphere().unit_vector()
    }
    pub fn set_x(&mut self, x:f64){
        self.x = x
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn set_y(&mut self, y:f64){
        self.y = y
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn set_z(&mut self, z:f64){
        self.z = z
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn unit_vector(&self) -> Self{
        let length = self.length();
        Point3D::new(self.x / length, self.y / length, self.z / length)
    }
    fn length(&self) -> f64{
        self.distance(&Point3D::new(0.0, 0.0, 0.0))
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn distance(&self, other:&Self) -> f64{
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        let dz = self.z - other.z();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    pub fn dot(&self, other: &Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Point3D) -> Point3D {
        Point3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }
}

impl Add for Point3D{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Mul<f64> for Point3D{
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Mul<Point3D> for Point3D {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Div<Point3D> for Point3D {
    type Output = Point3D;

    fn div(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
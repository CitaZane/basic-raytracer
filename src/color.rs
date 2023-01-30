use std::ops::{Mul,Add};
#[derive( Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color{
    pub fn new(r: f64, g:f64, b:f64) -> Self{
        Self{r,g,b}
    }
    pub fn red()->Self{
        Color { r: 1., g:0., b: 0. }
    }
    pub fn white()->Self{
        Color { r: 1., g:1., b: 1. }
    }
    pub fn black()->Self{
        Color { r: 0., g:0., b: 0. }
    }
    pub fn gray()->Self{
        Color { r: 0.4, g:0.4, b: 0.4 }
    }
    pub fn green()->Self{
        Color { r: 0.2, g:0.9, b: 0.1 }
    }
    pub fn write(&mut self, samples_per_pixel: i32){
        let scale = 1.0 / samples_per_pixel as f64;
        self.flatten_by_scale(scale);
        self.clamp(0.0, 0.999);

        let red = (255.999* self.r )as i32;
        let green = (255.999 * self.g) as i32;
        let blue = (255.999 * self.b) as i32;
        println!("{red} {green} {blue}");
    }
    fn flatten_by_scale(&mut self, scale:f64){
        self.r = (self.r * scale).sqrt();
        self.g = (self.g * scale).sqrt();
        self.b = (self.b * scale).sqrt();
    }
    fn clamp(&mut self, min:f64, max:f64){
        self.r = clamp(self.r, min, max);
        self.g = clamp(self.g, min, max);
        self.b = clamp(self.b, min, max);
    }
}

impl Add for Color{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f64> for Color{
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for Color{
    type Output = Self;
    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

fn clamp(x: f64, min:f64, max:f64)-> f64{
    if x<min {return min}
    if x>max{return max}
    x
}
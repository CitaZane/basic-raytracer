use crate::color::Color;

pub struct Image{
    pub width:usize,
    pub height:usize,
    pub samples_per_pixel:i32,
    pub pixels:Vec<Color>
}

impl Image{
    pub fn new(width:usize, height:usize)-> Self{
       Self { 
        width, 
        height, 
        pixels: vec![Color::new(0.,0.,0.); width*height], 
        samples_per_pixel:25
    } 
    }
    pub fn set_pixel(&mut self, x:usize, y:usize, color:Color){
        let i = x + y  * self.width;
        self.pixels[i]= color;
    }
    pub fn save_image(&mut self){
        self.print_header();
        self.pixels.iter_mut().for_each(|pixel|pixel.write(self.samples_per_pixel))
    }
    fn print_header(&self){
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!{"255"}
    }
}
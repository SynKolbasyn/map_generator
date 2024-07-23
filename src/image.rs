use std::path::Path;
use image::{GrayImage, Luma};
use crate::noise::noise_2d::Noise2D;

pub struct Image {
    image_buffer: GrayImage,
}


impl Image {
    fn new(image_buffer: GrayImage) -> Self {
        Self {
            image_buffer,
        }
    }
    
    pub fn from(noise2d: Noise2D) -> Self {
        let image_buffer: GrayImage = GrayImage::from_par_fn(
            noise2d.width(),
            noise2d.height(),
            |x: u32, y: u32| -> Luma<u8> {
                noise2d.get_pixel(x, y)
            },
        );
        
        Self::new(image_buffer)
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        self.image_buffer.save(path).unwrap()
    }
}

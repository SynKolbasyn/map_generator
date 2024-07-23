use image::Luma;
use rayon::prelude::*;

use crate::noise::{Noise, Resolution};


pub struct Noise2D {
    width: u32,
    height: u32,
    noise2d: Vec<Vec<f64>>,
}


impl Noise2D {
    fn new(width: u32, height: u32, noise2d: Vec<Vec<f64>>) -> Self {
        Self {
            width,
            height,
            noise2d,
        }
    }
    
    pub fn from(resolution: Resolution, octaves: u32, width: u32, height: u32) -> Self {
        let noise: Noise = Noise::from(resolution, octaves);
        
        let width_f: f64 = width.into();
        let height_f: f64 = height.into();
        
        let noise_2d: Vec<Vec<f64>> = (0..height).into_par_iter()
            .map(|y: u32| -> Vec<f64> {
                (0..width).into_par_iter()
                    .map(|x: u32| -> f64 {
                        let x: f64 = (x * resolution.x).into();
                        let y: f64 = (y * resolution.y).into();
                        noise.get_noise(x / width_f, y / height_f)
                    }).collect()
            }).collect();
        
        Self::new(
            width,
            height,
            noise_2d
        )
    }
    
    pub fn get_pixel(&self, x: u32, y: u32) -> Luma<u8> {
        Luma([(self.noise2d[y as usize][x as usize] * 255.0) as u8])
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

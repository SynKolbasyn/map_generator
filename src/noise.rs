use std::ops::Mul;
use rayon::prelude::*;

use crate::math::Grid;
use crate::math::point::Point;

pub struct Noise {
    resolution: Resolution,
    octaves: u32,
    grids: Vec<Grid>,
}


impl Noise {
    fn new(resolution: Resolution, octaves: u32, grids: Vec<Grid>) -> Self {
        Self {
            resolution,
            octaves,
            grids,
        }
    }
    
    pub fn from(resolution: Resolution, octaves: u32) -> Self {
        let grids: Vec<Grid> = (0..octaves).into_par_iter()
            .map(|octave| -> Grid {
                let multiplier: u32 = 2_u32.pow(octave); 
                Grid::from(
                    resolution.x * multiplier,
                    resolution.y * multiplier
                )
            }).collect();
        
        Self::new(
            resolution,
            octaves,
            grids,
        )
    }
    
    pub fn get_noise(&self, point: Point) -> f64 {
        self.grids.par_iter().map(|grid| -> f64 { 0.0 }).sum()
    }
}


#[derive(Copy, Clone)]
pub struct Resolution {
    x: u32,
    y: u32,
}


impl Resolution {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}

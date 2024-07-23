pub mod noise_2d;

use rayon::prelude::*;

use crate::math::Grid;
use crate::math::point::Point;


pub struct Noise {
    grids: Vec<Grid>,
}


impl Noise {
    fn new(grids: Vec<Grid>) -> Self {
        Self {
            grids,
        }
    }
    
    pub fn from(resolution: Resolution, octaves: u32) -> Self {
        let grids: Vec<Grid> = (0..octaves).into_par_iter()
            .map(|octave| -> Grid {
                let multiplier: u32 = 2_u32.pow(octave);
                Grid::from(
                    (resolution.x + 1) * multiplier,
                    (resolution.y + 1) * multiplier,
                    octave as i32,
                )
            }).collect();
        
        Self::new(grids)
    }
    
    pub fn get_noise<X: Into<f64>, Y: Into<f64>>(&self, x: X, y: Y) -> f64 {
        let point: Point = Point::from(x, y);
        self.grids.par_iter()
            .map(|grid: &Grid| -> f64 {
                grid.process(point)
            }).sum()
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

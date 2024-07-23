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
    
    pub fn get_noise<X: Into<f64>, Y: Into<f64>>(&self, x: X, y: Y) -> f64 {
        let point: Point = Point::from(x, y);
        self.grids.par_iter()
            .map(|grid: &Grid| -> f64 {
                grid.process(point) * 0.5_f64.powi((grid.x() / self.resolution.x) as i32)
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

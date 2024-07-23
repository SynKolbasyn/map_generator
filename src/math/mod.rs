pub mod point;
pub mod vector;
mod grid_point;

use rayon::prelude::*;
use crate::math::grid_point::GridPoint;


pub struct Grid {
    x: u32,
    y: u32,
    points: Vec<Vec<GridPoint>>
}


impl Grid {
    fn new(x: u32, y: u32, points: Vec<Vec<GridPoint>>) -> Self {
        Self {
            x,
            y,
            points,
        }
    }
    
    pub fn from<X: Into<u32> + Copy + Sync, Y: Into<u32> + Copy + Sync>(x: X, y: Y) -> Self {
        let points: Vec<Vec<GridPoint>> = (0..y.into()).into_par_iter()
            .map(|y: u32| -> Vec<GridPoint> {
                (0..x.into()).into_par_iter()
                    .map(|x: u32| -> GridPoint {
                        GridPoint::from(x, y)
                    }).collect()
            }).collect();
        
        Self::new(x.into(), y.into(), points)
    }
    
    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

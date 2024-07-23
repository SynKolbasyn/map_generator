pub mod point;
pub mod vector;
pub mod grid_point;


use rayon::join;
use rayon::prelude::*;

use crate::math::point::Point;
use crate::math::grid_point::GridPoint;
use crate::math::vector::Vector;


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
    
    pub fn process(&self, point: Point) -> f64 {
        let tx: f64 = point.x - point.x.floor();
        let ty: f64 = point.y - point.y.floor();
        
        let lbp: GridPoint = self.points[point.y.floor() as usize][point.x.floor() as usize];
        let rbp: GridPoint = self.points[point.y.floor() as usize][point.x.ceil() as usize];
        let lup: GridPoint = self.points[point.y.ceil() as usize][point.x.floor() as usize];
        let rup: GridPoint = self.points[point.y.ceil() as usize][point.x.ceil() as usize];
        
        let (a, b): (f64, f64) = join(
            || -> f64 {
                let a: f64 = lbp.vector() * Vector::new(lbp.point(), point);
                let b: f64 = rbp.vector() * Vector::new(rbp.point(), point);
                Self::interpolate(a, b, tx)
            },
            || -> f64 {
                let a: f64 = lup.vector() * Vector::new(lup.point(), point);
                let b: f64 = rup.vector() * Vector::new(rup.point(), point);
                Self::interpolate(a, b, tx)
            },
        );
        println!("{a} {b}");
        
        Self::interpolate(a, b, ty)
    }

    fn interpolate(a: f64, b: f64, t: f64) -> f64 {
        a + Self::smoother_step(t) * (b - a)
    }

    fn smoother_step(t: f64) -> f64 {
        t * t * t * (t * (6.0 * t - 15.0) - 10.0)
    }
    
    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

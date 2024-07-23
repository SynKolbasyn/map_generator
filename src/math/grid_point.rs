use rand::prelude::*;

use crate::math::point::Point;


pub struct GridPoint {
    point: Point,
    height: f64,
}


impl GridPoint {
    fn new(point: Point, height: f64) -> Self {
        Self {
            point,
            height,
        }
    }

    pub fn from<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
        Self::new(
            Point::from(x, y),
            thread_rng().gen_range(0.0..=1.0),
        )
    }
}

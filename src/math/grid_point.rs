use crate::math::point::Point;
use crate::math::vector::Vector;


#[derive(Copy, Clone)]
pub struct GridPoint {
    point: Point,
    unit_vector: Vector,
}


impl GridPoint {
    fn new(point: Point, unit_vector: Vector) -> Self {
        Self {
            point,
            unit_vector,
        }
    }

    pub fn from<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
        Self::new(
            Point::from(x, y),
            Vector::random_unit(),
        )
    }
    
    pub fn point(&self) -> Point {
        self.point
    }
    
    pub fn vector(&self) -> Vector {
        self.unit_vector
    }
}

use crate::noise::point::Point;
use crate::noise::vector::{Cos, Vector};


#[derive(Copy, Clone)]
pub struct GridPoint {
    pub point: Point,
    pub unit_vector: Vector,
    pub vector_to_point: Vector
}


impl Default for GridPoint {
    fn default() -> Self {
        Self::new(
            Point::default(),
            Vector::random_unit(),
            Vector::default(),
        )
    }
}


impl GridPoint {
    pub fn new(point: Point, unit_vector: Vector, vector_to_point: Vector) -> Self {
        Self {
            point,
            unit_vector,
            vector_to_point,
        }
    }

    pub fn interpolate(&self, rhs: Self) -> f64 {
        let mut t = self.unit_vector * self.vector_to_point;
        let a = self.unit_vector.val();
        let b = rhs.unit_vector.val();

        

        a + t * (b - a)
    }
}

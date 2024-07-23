use crate::math::point::Point;


#[derive(Copy, Clone)]
pub struct Vector {
    a: Point,
    b: Point,
}


impl Vector {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            a,
            b,
        }
    }
    
    pub fn x(&self) -> f64 {
        self.b.x - self.a.x
    }

    pub fn y(&self) -> f64 {
        self.b.y - self.a.y
    }
    
    pub fn len(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

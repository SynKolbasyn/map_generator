use std::f64::consts::PI;

use std::ops::Mul;

use rand::prelude::*;

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
    
    pub fn random_unit() -> Self {
        Self::random().to_unit()
    }
    
    fn random() -> Self {
        let theta: f64 = thread_rng().gen_range(0.0..(PI * 2.0));
        Self::new(
            Point::from(0, 0),
            Point::from(theta.cos(), theta.sin()),
        )
        // Self::new(
        //     Point::random(),
        //     Point::random(),
        // )
    }
    
    fn to_unit(&self) -> Self {
        let len: f64 = self.len();
        Self::new(
            Point::from(0, 0),
            Point::from(self.x() / len, self.y() / len),
        )
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


impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}

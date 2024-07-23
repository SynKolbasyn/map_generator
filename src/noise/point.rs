use std::ops::{Add, Mul, Sub};
use rand::random;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Default for Point {
    fn default() -> Self {
        Self::new(
            f64::default(),
            f64::default(),
        )
    }
}


impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
        )
    }
}


impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
        )
    }
}


impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
        )
    }
}


impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
    
    pub fn from<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
        Self::new(x.into(), y.into())
    }
    
    pub fn random() -> Self {
        Self::new(
            random(),
            random(),
        )
    }
}

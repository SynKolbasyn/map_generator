use std::ops::{Add, Mul};

use crate::noise::point::Point;


pub trait Cos<T> {
    fn cos(self, rhs: T) -> f64;
}


#[derive(Copy, Clone)]
pub struct Vector {
    a: Point,
    b: Point,
}


impl Default for Vector {
    fn default() -> Self {
        Self::new(
            Point::default(),
            Point::default(),
        )
    }
}


impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a + rhs.a,
            self.b + rhs.b,
        )
    }
}


impl Mul<Self> for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}


impl Mul<&Self> for Vector {
    type Output = f64;

    fn mul(self, rhs: &Self) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}


impl Cos<Self> for Vector {
    fn cos(self, rhs: Self) -> f64 {
        (self * rhs) / (self.len() * rhs.len())
    }
}


impl Cos<&Self> for Vector {
    fn cos(self, rhs: &Self) -> f64 {
        (self * rhs) / (self.len() * rhs.len())
    }
}


impl Vector {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            a,
            b,
        }
    }

    pub fn new_unit(a: Point, b: Point) -> Self {
        Self::new(a, b).unit_vector()
    }
    
    pub fn random() -> Self {
        Self::new(
            Point::random(),
            Point::random(),
        )
    }

    pub fn random_unit() -> Self {
        Self::random().unit_vector()
    }

    pub fn x(&self) -> f64 {
        (self.b - self.a).x
    }

    pub fn y(&self) -> f64 {
        (self.b - self.a).y
    }
    
    pub fn len(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
    
    pub fn unit_vector(&self) -> Self {
        Self::new(
            Point::default(),
            Point::new(self.x() / self.len(), self.y() / self.len()),
        )
    }
    
    pub fn val(&self) -> f64 {
        self.cos(Vector::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)))
    }
}

use rand::prelude::*;


#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
    
    pub fn from<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
        Self::new(x.into(), y.into())
    }
    
    #[allow(dead_code)]
    pub fn random() -> Self {
        Self::new(random(), random())
    }
}

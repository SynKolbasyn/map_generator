pub mod point;
pub mod vector;
mod grid_point;
mod grid;


use anyhow::Result;


use crate::noise::grid::Grid;
use crate::noise::point::Point;


pub struct Noise {
    resolution: u32,
    octaves: u128,
    grid: Grid,
}


impl Default for Noise {
    fn default() -> Self {
        Self::new(
            10,
            1,
        )
    }
}


impl Noise {
    pub fn new(resolution: u32, octaves: u128) -> Self {
        Self {
            resolution,
            octaves,
            grid: Grid::new(resolution),
        }
    }

    pub fn get_noise(&mut self, point: Point) -> Result<f64> {
        self.grid.set_point(point)?;
        Ok(self.grid.interpolate(point)?)
    }
}

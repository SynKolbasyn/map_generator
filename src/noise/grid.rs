use anyhow::{Result, Context};

use rayon::prelude::*;

use crate::noise::grid_point::GridPoint;
use crate::noise::point::Point;
use crate::noise::vector::Vector;


pub struct Grid {
    grid: Vec<Vec<GridPoint>>,
}


impl Grid {
    pub fn new(size: u32) -> Self {
        let grid: Vec<Vec<GridPoint>> = (0..size)
            .into_par_iter()
            .map(|x| -> Vec<GridPoint> {
                (0..size)
                    .into_par_iter()
                    .map(|y| -> GridPoint {
                        GridPoint::new(
                            Point::from(x, y),
                            Vector::random_unit(),
                            Vector::default(),
                        )
                    }).collect()
            }).collect();
        
        Self {
            grid,
        }
    }
    
    pub fn set_point(&mut self, point: Point) -> Result<()> {
        let (p1, p2, p3, p4): (GridPoint, GridPoint, GridPoint, GridPoint) = self.get_near_points(point)?;

        self.grid[point.y.floor() as usize][point.x.floor() as usize]
            .vector_to_point = Vector::new(p1.point, point);
        self.grid[point.y.floor() as usize][point.x.ceil() as usize]
            .vector_to_point = Vector::new(p2.point, point);
        self.grid[point.y.ceil() as usize][point.x.floor() as usize]
            .vector_to_point = Vector::new(p3.point, point);
        self.grid[point.y.ceil() as usize][point.x.ceil() as usize]
            .vector_to_point = Vector::new(p4.point, point);
        
        Ok(())
    }

    pub fn interpolate(&self, point: Point) -> Result<f64> {
        let (p1, p2, p3, p4): (GridPoint, GridPoint, GridPoint, GridPoint) = self.get_near_points(point)?;

        let a1: f64 = p1.unit_vector * p1.vector_to_point;
        let b1: f64 = p2.unit_vector * p2.vector_to_point;
        let a2: f64 = p3.unit_vector * p3.vector_to_point;
        let b2: f64 = p4.unit_vector * p4.vector_to_point;

        let mut t1: f64 = point.x - point.x.floor();
        let mut t2: f64 = point.y - point.y.floor();
        t1 = t1 * t1 * t1 * (t1 * (6.0 * t1 - 15.0) + 10.0);
        t2 = t2 * t2 * t2 * (t2 * (6.0 * t2 - 15.0) + 10.0);

        let a3: f64 = a1 + t1 * (b1 - a1);
        let b3: f64 = a2 + t1 * (b2 - a2);

        Ok(a3 + t2 * (b3 - a3))
    }

    fn get_near_points(&self, point: Point) -> Result<(GridPoint, GridPoint, GridPoint, GridPoint)> {
        let p1: GridPoint = self.grid.get(point.y.floor() as usize)
            .context("Error in getting the nearest points")?.get(point.x.floor() as usize)
            .context("Error in getting the nearest points")?.clone();
        let p2: GridPoint = self.grid.get(point.y.floor() as usize)
            .context("Error in getting the nearest points")?.get(point.x.ceil() as usize)
            .context("Error in getting the nearest points")?.clone();
        let p3: GridPoint = self.grid.get(point.y.ceil() as usize)
            .context("Error in getting the nearest points")?.get(point.x.floor() as usize)
            .context("Error in getting the nearest points")?.clone();
        let p4: GridPoint = self.grid.get(point.y.ceil() as usize)
            .context("Error in getting the nearest points")?.get(point.x.ceil() as usize)
            .context("Error in getting the nearest points")?.clone();

        Ok((p1, p2, p3, p4))
    }
}

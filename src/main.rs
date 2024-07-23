mod noise;
mod image;

use rand::prelude::*;
use crate::noise::Noise;
use crate::noise::point::Point;


fn main() {
    let mut noise: Noise = Noise::new(3, 1);
    // for i in 0..250 {
    //     for j in 0..250 {
    //         noise.get_noise(Point::new(i as f64 / 100.0, j as f64 / 100.0)).unwrap();
    //     }
    // }
    let mut m: f64 = 0.0;
    loop {
        let res: f64 = noise.get_noise(Point::new(thread_rng().gen_range(0.0..2.0), thread_rng().gen_range(0.0..2.0))).unwrap().abs();
        if m < res {
            m = res;
            println!("{m}");
        }
    }
    match noise.get_noise(Point::new(1.7, 1.8)) {
        Ok(result) => println!("{result}"),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}

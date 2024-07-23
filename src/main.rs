use rand::random;
use crate::noise::{Noise, Resolution};

mod math;
mod noise;


fn main() {
    let resolution: Resolution = Resolution::new(3, 3);
    let octaves: u32 = 3;
    
    let noise: Noise = Noise::from(resolution, octaves);
    let mut m: f64 = 0.0;
    loop {
        let n: f64 = noise.get_noise(random::<f64>(), random::<f64>());
        if m < n.abs() {
            m = n.abs();
            println!("{m}");
        }
    }
}

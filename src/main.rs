use crate::noise::{Noise, Resolution};

mod math;
mod noise;


fn main() {
    let resolution: Resolution = Resolution::new(3, 3);
    let octaves: u32 = 3;
    
    let noise: Noise = Noise::from(resolution, octaves);
    println!("{}", noise.get_noise(1.5, 0.4))
}

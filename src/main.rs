mod math;
mod noise;
mod image;


use crate::image::Image;
use crate::noise::Resolution;
use crate::noise::noise_2d::Noise2D;


fn main() {
    let resolution: Resolution = Resolution::new(12, 12);
    let octaves: u32 = 10;
    let (width, height): (u32, u32) = (1080, 1080);

    let noise_2d: Noise2D = Noise2D::from(resolution, octaves, width, height);

    let image: Image = Image::from(noise_2d);
    image.save("noise.png")
}

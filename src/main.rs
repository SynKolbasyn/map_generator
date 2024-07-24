mod math;
mod noise;
mod image;


use std::fs::create_dir_all;
use std::io::{stdin, stdout, Write};

use crate::image::Image;
use crate::noise::Resolution;
use crate::noise::noise_2d::Noise2D;


fn main() {
    let mut idx: u128 = 0;
    loop {
        let mut select: String = String::new();
        print!("Do you want to use custom settings? (default - no): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut select).unwrap();
        select = select.trim().to_string();

        if select.is_empty() || select == "n" || select == "no" {
            from_template(idx);
        }
        else {
            from_user(idx);
        }
        idx += 1;
    }
}


fn from_template(idx: u128) {
    let resolution: Resolution = Resolution::new(5, 5);
    let octaves: u32 = 5;
    let (width, height): (u32, u32) = (640, 640);

    println!("Template setting:");
    println!("resolution x: {}", resolution.x);
    println!("resolution y: {}", resolution.y);
    println!("octaves: {octaves}");
    println!("width: {width}");
    println!("height: {height}");

    let noise_2d: Noise2D = Noise2D::from(resolution, octaves, width, height);

    let image: Image = Image::from(noise_2d);
    create_dir_all("./images/").unwrap();
    image.save(format!("./images/noise_template_{idx}.png"));
}


fn from_user(idx: u128) {
    let mut resolution_x: String = String::new();
    let mut resolution_y: String = String::new();
    let mut octaves: String = String::new();
    let mut width: String = String::new();
    let mut height: String = String::new();

    print!("Enter resolution x: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut resolution_x).unwrap();

    print!("Enter resolution y: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut resolution_y).unwrap();

    print!("Enter num of octaves: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut octaves).unwrap();

    print!("Enter width of picture: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut width).unwrap();

    print!("Enter height of picture: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut height).unwrap();

    resolution_x = resolution_x.trim().to_string();
    resolution_y = resolution_y.trim().to_string();
    octaves = octaves.trim().to_string();
    width = width.trim().to_string();
    height = height.trim().to_string();

    let resolution: Resolution = Resolution::new(resolution_x.parse().unwrap(), resolution_y.parse().unwrap());
    let octaves: u32 = octaves.parse().unwrap();
    let (width, height): (u32, u32) = (width.parse().unwrap(), height.parse().unwrap());

    let noise_2d: Noise2D = Noise2D::from(resolution, octaves, width, height);

    let image: Image = Image::from(noise_2d);
    create_dir_all("./images/").unwrap();
    image.save(format!("./images/noise_{resolution_x}_{resolution_y}_{octaves}_{width}_{height}_{idx}.png"));
}

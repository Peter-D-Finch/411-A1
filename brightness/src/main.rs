extern crate image;
use crate::image::GenericImageView;
use crate::image::Pixel;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let img = image::open(&args[1]).unwrap();
    let num_px = (img.width() * img.height()) as i64;
    let mut sum: i64 = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let pixel = img.get_pixel(i, j).to_luma();
            sum = sum + (pixel.0[0] as i64);
        }
    }
    let avg: f64 = (sum as f64) / (num_px as f64) / (255 as f64);
    println!("{:.3}", avg);
}
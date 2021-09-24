extern crate image;
use crate::image::GenericImageView;
use crate::image::Pixel;
use std::env;


fn main() {
    // Collecting command line arguments 
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = &args[1];

    // Checking to make sure the file type is a graymap
    let mut ftype = filename.split('.');
    ftype.next();
    let file_type = ftype.next().unwrap();
    if file_type != "ppm" && file_type != "pgm" && file_type != "pbm" {
        eprintln!("Not Graymap file.");
        std::process::exit(0);
    }

    // Opening image using image crate
    let img = image::open(&args[1]).unwrap();
    let denom = img.get_pixel(0,0).0[3];

    // Defining # pixels in image and accumulator i64 to sum all the pixel values
    let num_px = (img.width() * img.height()) as i64;
    let mut sum: i64 = 0;

    // Walk through each pixel in the image adding it's value to accumulator
    for i in 0..img.width() {
        for j in 0..img.height() {
            // Convert Rgba<u8> to Luma<u8>
            let pixel = img.get_pixel(i, j).to_luma();
            // Convert u8 value in Luma pixel to i64 and add to accumulator
            sum = sum + (pixel.0[0] as i64);
        }
    }

    // Find average value 0-denom and then divide by denom to get value 0-1
    let avg: f64 = (sum as f64) / (num_px as f64) / (denom as f64);
    println!("{:.3}", avg);
}
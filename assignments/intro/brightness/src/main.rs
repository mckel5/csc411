use csc411_image::{GrayImage, Read};
use std::env;

fn main() {
    let input = env::args().nth(1);
    // Automatically reads from stdin if input is None
    let img = GrayImage::read(input.as_deref()).unwrap();
    assert!(env::args().len() <= 2);

    // Some PNM/PGM images do not use the full 0-255 scale, so we use a
    // variable denominator for the max brightness value
    let denominator = img.denominator;
    let total_brightness = img.pixels.iter().fold(0, |acc, pixel| acc + (pixel.value as u64));
    let average_brightness = (total_brightness as f64) / ((img.width * img.height) as f64);
    let average_brightness_limited = average_brightness / denominator as f64;

    println!("{:.3}", average_brightness_limited);
}

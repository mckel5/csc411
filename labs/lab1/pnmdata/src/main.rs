use csc411_image::{GrayImage, Read};
use std::env;

fn main() {
    let input = env::args().nth(1);
    assert!(env::args().len() == 2);

    let img = GrayImage::read(input.as_deref()).unwrap();
    let h = img.height;
    let w = img.width;
    println!("{:?} has dimensions width {} and height {}", input, w, h);
}

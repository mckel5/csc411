use std::cmp;
use std::env;
use std::process;
use array2::Array2;
use csc411_image::{RgbImage, Read, Rgb};

fn main() {
    let filenames = (env::args().nth(1), env::args().nth(2));
    let image1 = RgbImage::read(filenames.0.as_deref()).unwrap();
    let image2 = RgbImage::read(filenames.1.as_deref()).unwrap();

    if image1.width.abs_diff(image2.width) > 1 {
        eprintln!("Width mismatch: image 1 has width {} but image 2 has width {}", image1.width, image2.width);
        println!("1.0");
        process::exit(1);
    }

    if image1.height.abs_diff(image2.height) > 1 {
        eprintln!("Height mismatch: image 1 has height {} but image 2 has height {}", image1.height, image2.height);
        println!("1.0");
        process::exit(1);
    }

    let denominator1 = image1.denominator;
    let denominator2 = image2.denominator;

    let min_width = cmp::min(image1.width, image2.width) as usize;
    let min_height = cmp::min(image1.height, image2.height) as usize;

    let array1 = Array2::from_row_major(image1.pixels, image1.width as usize);
    let array2 = Array2::from_row_major(image2.pixels, image2.width as usize);

    let mut total_diff = 0_f64;

    for row in 0..min_height {
        for col in 0..min_width {
            let px1 = array1.get(row, col).unwrap();
            let px2 = array2.get(row, col).unwrap();

            let (r1, g1, b1) = normalize_rgb(px1, denominator1);
            let (r2, g2, b2) = normalize_rgb(px2, denominator2);

            let r_diff = (r1 - r2).powi(2);
            let g_diff = (g1 - g2).powi(2);
            let b_diff = (b1 - b2).powi(2);

            total_diff += r_diff + g_diff + b_diff;
        }
    }

    let root_mean_square_difference = (total_diff / (3 * min_width * min_height) as f64).sqrt();
    println!("{:.4}", root_mean_square_difference);
}

fn normalize_rgb(pixel: &Rgb, denominator: u16) -> (f64, f64, f64) {
    let d = denominator as f64;
    (
        pixel.red as f64 / d,
        pixel.green as f64 / d,
        pixel.blue as f64 / d
    )
}
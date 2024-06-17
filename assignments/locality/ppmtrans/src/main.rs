use array2::Array2;
use csc411_image::{Rgb, RgbImage, Read, Write};
use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Row-major
    #[clap(long = "row-major")]
    row_major: bool,
    // Column-major
    #[clap(long = "col-major")]
    column_major: bool,
    // Rotation
    #[clap(long = "rotate")]
    rotate: Option<u32>,
    // Flip
    #[clap(long = "flip")]
    flip: Option<String>,
    // Transposition
    #[clap(long = "transpose")]
    transpose: bool,
    // Benchmark
    #[clap(long = "benchmark")]
    benchmark: bool,
    // Filename
    filename: Option<String>,
}

type ImageArray = Array2<Rgb>;

fn main() {
    let args = Args::parse();
    let Args {
        row_major: _,
        column_major,
        rotate: _rotate,
        flip: _flip,
        transpose: _transpose,
        benchmark,
        filename
    } = args;
    let input_image = RgbImage::read(filename.as_deref()).unwrap();
    let denominator = input_image.denominator;
    let mut image_array = image_to_array2(input_image);

    let now = Instant::now();

    image_array = match _rotate {
        Some(angle) => rotate(image_array, angle, column_major),
        None => image_array
    };

    image_array = match _flip {
        Some(direction) => flip(image_array, direction, column_major),
        None => image_array
    };

    image_array = match _transpose {
        true => transpose(image_array, column_major),
        false => image_array
    };

    let elapsed = now.elapsed();

    if benchmark { eprintln!("{:.2?}", elapsed); }

    let output_image = array2_to_image(image_array, denominator);
    output_image.write(None).expect("Error writing file.");
}

fn image_to_array2(image: RgbImage) -> ImageArray {
    Array2::from_row_major(image.pixels, image.width as usize)
}

fn array2_to_image(array2: ImageArray, denominator: u16) -> RgbImage {
    RgbImage {
        pixels: array2.iter_row_major().map(|(_, _, rgb)| rgb).collect(),
        width: array2.width as u32,
        height: array2.height as u32,
        denominator,
    }
}

fn rotate(image: ImageArray, angle: u32, column_major: bool) -> ImageArray {
    assert_eq!(angle % 90, 0, "Rotation angle must be a multiple of 90.");
    let width = image.width;
    let height = image.height;
    // Adjust angle to fall within [0, 360)
    let adjusted_angle = angle % 360;
    match adjusted_angle {
        0 => image,
        90 => transform(image, |row, column| (column, height - row - 1), true, column_major),
        180 => transform(image, |row, column| (height - row - 1, width - column - 1), false, column_major),
        270 => transform(image, |row, column| (width - column - 1, row), true, column_major),
        _ => image
    }
}

fn flip(image: ImageArray, direction: String, column_major: bool) -> ImageArray {
    assert!(direction.as_str() == "horizontal" || direction.as_str() == "vertical", "Flip direction must be 'horizontal' or 'vertical'.");
    let width = image.width;
    let height = image.height;
    match direction.as_str() {
        "horizontal" => transform(image, |row, column| (row, width - column - 1), false, column_major),
        "vertical" => transform(image, |row, column| (height - row - 1, column), false, column_major),
        _ => image
    }
}

fn transpose(image: ImageArray, column_major: bool) -> ImageArray {
    transform(image, |row, column| (column, row), true, column_major)
}

fn transform(image: ImageArray, f: impl Fn(usize, usize) -> (usize, usize), swap_dimensions: bool, column_major: bool) -> ImageArray {
    let (new_width, new_height) = match swap_dimensions {
        true => (image.height, image.width),
        false => (image.width, image.height)
    };
    let blank_pixel = Rgb { red: 0, green: 0, blue: 0 };
    let mut output_array: Array2<Rgb> = Array2::from_single_value(blank_pixel, new_width, new_height);

    // I do not like this code duplication but I am not yet knowledgeable enough to get Rust to
    // accept *either* an Array2IterRowMajor or Array2IterColumnMajor
    if column_major {
        for (row, column, pixel) in image.iter_col_major() {
            let (new_row, new_column) = f(row, column);
            let new_pixel: &mut Rgb = output_array.get_mut(new_row, new_column).unwrap();
            *new_pixel = pixel;
        }
    } else {
        for (row, column, pixel) in image.iter_row_major() {
            let (new_row, new_column) = f(row, column);
            let new_pixel: &mut Rgb = output_array.get_mut(new_row, new_column).unwrap();
            *new_pixel = pixel;
        }
    }

    output_array
}

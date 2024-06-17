use csc411_image::{GrayImage, Read};
use std::env;
use std::process::exit;
use array2::Array2;

fn main() {
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    assert!(env::args().len() <= 2, "Format: sudoku [path/to/img]");

    assert_eq!(img.denominator, 9, "Invalid PGM denominator");
    assert_eq!(img.width, 9, "Invalid width");
    assert_eq!(img.height, 9, "Invalid height");

    let board = grayimage_to_array2(img);

    if !(all_rows_valid(&board) && all_columns_valid(&board) && all_squares_valid(&board)) {
        exit(1);
    }

    exit(0);
}

/// Convert a PGM-like GrayImage to an Array2 whose elements represent the brightness of each pixel
fn grayimage_to_array2(img: GrayImage) -> Array2<u16> {
    Array2::from_row_major(img
                               .pixels
                               .iter()
                               .map(|pixel| pixel.value)
                               .collect(),
                           9)
}

/// Check if each row on the board is valid
fn all_rows_valid(board: &Array2<u16>) -> bool {
    let rows = board
        .iter_row_major()
        .collect::<Vec<_>>()
        .chunks(9)
        .map(|chunk| chunk.into())
        .collect::<Vec<Vec<_>>>();

    for row in rows {
        if !set_of_nine_values_valid(row) { return false; }
    }

    true
}

/// Check if each column on the board is valid
fn all_columns_valid(board: &Array2<u16>) -> bool {
    let columns = board
        .iter_col_major()
        .collect::<Vec<_>>()
        .chunks(9)
        .map(|chunk| chunk.into())
        .collect::<Vec<Vec<_>>>();

    for column in columns {
        if !set_of_nine_values_valid(column) { return false; }
    }

    true
}

/// Check if each 3x3 "sub-square" on the board is valid
fn all_squares_valid(board: &Array2<u16>) -> bool {
    // Since the size of the sudoku boards never changes, I have hard-coded these values for
    // simplicity
    let start_coordinates = vec![
        (0, 0), (0, 3), (0, 6),
        (3, 0), (3, 3), (3, 6),
        (6, 0), (6, 3), (6, 6),
    ];

    for coordinate in start_coordinates {
        let mut values = Vec::with_capacity(9);

        for row_offset in 0..3 {
            for column_offset in 0..3 {
                values.push(board
                    .get(coordinate.0 + row_offset, coordinate.1 + column_offset)
                    .cloned()
                    .unwrap());
            }
        }

        if !set_of_nine_values_valid(values) { return false; }
    }

    true
}

/// Check if a given Vec of nine values follows Sudoku rules
/// (i.e. contains every digit 1-9 with no duplicates)
fn set_of_nine_values_valid(mut values: Vec<u16>) -> bool {
    values.sort();
    values.dedup();
    values.len() == 9 && values.iter().sum::<u16>() == 45
}
use array2::Array2;
use csc411_image::{Read, RgbImage, Write};
use crate::normalize::{normalize, denormalize};
use csc411_rpegio;
use crate::bitpack::{pack, unpack};
use crate::component::{Component, component_to_rgb, rgb_to_component};
use crate::dct::{CosineBlock, to_cosine_space, to_pixel_space};
use crate::quantize::{dequantize, quantize, QuantizedBlock};

/// Compress a PPM image. Writes the result to stdout.
///
/// # Arguments:
/// * `filename`: File to read from (`None` for stdin)
pub fn compress(filename: Option<&str>) {
    // Read image
    let input = RgbImage::read(filename.as_deref()).unwrap();
    let denominator = input.denominator;

    // RGB
    let image = Array2::from_row_major(input.pixels, input.width as usize);

    // Trim image
    let trimmed = trim_image(image);
    let new_width = trimmed.width;
    let new_height = trimmed.height;

    // Normalized RGB
    let normalized = Array2::from_row_major(
        trimmed.iter_row_major()
            .map(|rgb| normalize(rgb, denominator))
            .collect(),
        trimmed.width);

    // Component
    let componentized = Array2::from_row_major(
        normalized.iter_row_major()
            .map(|normal| rgb_to_component(normal))
            .collect(),
        normalized.width,
    );

    // 2x2 blocks
    let blocks = blockify(componentized, Component { y: 0.0, pb: 0.0, pr: 0.0 });

    // Discrete cosine transform
    let cosine_blocks: Vec<CosineBlock> = blocks.iter_row_major()
        .map(|block| to_cosine_space(block.clone()))
        .collect();

    // Quantization
    let quantized: Vec<QuantizedBlock> = cosine_blocks.iter()
        .map(|block| quantize(block.clone()))
        .collect();

    // Bitpack
    let packed_words: Vec<u64> = quantized.iter()
        .map(|block| pack(block.clone()))
        .collect();

    // Write binary
    let byte_slices: Vec<[u8; 4]> = packed_words.iter()
        .map(|word| (word.clone() as u32).to_be_bytes())
        .collect();

    csc411_rpegio::output_rpeg_data(&byte_slices, new_width, new_height).unwrap();
}

/// Deompress a bitpacked binary image. Writes the result to stdout.
///
/// # Arguments:
/// * `filename`: File to read from (`None` for stdin)
pub fn decompress(filename: Option<&str>) {
    // Read binary
    let (byte_slices, width, _) = csc411_rpegio::input_rpeg_data(filename).unwrap();

    let packed_words: Vec<u64> = byte_slices.iter()
        .map(|slice| u32::from_be_bytes(*slice) as u64)
        .collect();

    // Bitpack
    let unpacked_blocks: Vec<QuantizedBlock> = packed_words.iter()
        .map(|word| unpack(word.clone()))
        .collect();

    // Quantization
    let dequantized: Vec<CosineBlock> = unpacked_blocks.iter()
        .map(|block| dequantize(block.clone()))
        .collect();

    // 2x2 blocks
    let pixel_blocks: Array2<Array2<Component>> =
        Array2::from_row_major(
            dequantized.iter()
                .map(|block| to_pixel_space(block.clone()))
                .collect(),
            width / 2,
        );

    // Component
    let deblocked = deblockify(pixel_blocks, Component { y: 0.0, pb: 0.0, pr: 0.0 });

    // Normalized RGB
    let decomponentized = Array2::from_row_major(
        deblocked.iter_row_major()
            .map(|component| component_to_rgb(component))
            .collect(),
        deblocked.width,
    );

    // RGB
    let denormalized = Array2::from_row_major(
        decomponentized.iter_row_major()
            .map(|normal_rgb| denormalize(normal_rgb))
            .collect(),
        decomponentized.width,
    );

    // Write image
    let output = RgbImage {
        pixels: denormalized.iter_row_major().collect(),
        width: denormalized.width as u32,
        height: denormalized.height as u32,
        denominator: 255,
    };

    output.write(None).expect("Error writing PPM image");
}

/// Trim the last row of an array if the number of rows is odd.
/// Likewise for columns.
///
/// # Arguments:
/// * `array`: the `Array2` to be trimmed
pub fn trim_image<T>(array: Array2<T>) -> Array2<T> where T: Clone {
    if array.width % 2 == 0 && array.height % 2 == 0 {
        return array;
    }

    let mut new_array = array.clone();

    if array.width % 2 != 0 {
        let mut new_vec = new_array.iter_col_major().collect::<Vec<_>>();
        new_vec.truncate(new_array.width * new_array.height - new_array.height);
        new_array = Array2::from_col_major(new_vec, new_array.height);
    }

    if array.height % 2 != 0 {
        let mut new_vec = new_array.iter_row_major().collect::<Vec<_>>();
        new_vec.truncate(new_array.width * new_array.height - new_array.width);
        new_array = Array2::from_row_major(new_vec, new_array.width);
    }

    new_array
}

/// Transform an `Array2` into a set of 2x2 blocks.
/// The original values are not modified.
///
/// # Arguments
/// `array`: the `Array2` to be blockified
pub fn blockify<T>(array: Array2<T>, default_value: T) -> Array2<Array2<T>> where T: Clone {
    let mut blocks: Array2<Array2<T>> = Array2::from_single_value(
        Array2::from_single_value(default_value, 2, 2),
        array.width / 2, array.height / 2);

    for row in (0..array.height).step_by(2) {
        for col in (0..array.width).step_by(2) {
            let items = vec![
                array.get(row, col).cloned().unwrap(),
                array.get(row, col + 1).cloned().unwrap(),
                array.get(row + 1, col).cloned().unwrap(),
                array.get(row + 1, col + 1).cloned().unwrap(),
            ];

            let block = blocks.get_mut(row / 2, col / 2).unwrap();
            *block = Array2::from_row_major(items, 2);
        }
    }

    blocks
}

/// Flatten a blockified `Array2` into its original form.
/// The initial order of the elements is preserved.
///
/// # Arguments
/// `array`: the `Array2` to be deblockified
pub fn deblockify<T>(array: Array2<Array2<T>>, default_value: T) -> Array2<T> where T: Clone {
    let mut deblocked: Array2<T> = Array2::from_single_value(default_value, array.width * 2, array.height * 2);

    for row in 0..array.height {
        for col in 0..array.width {
            let mut item: &mut T;

            for i in 0..2 {
                for j in 0..2 {
                    item = deblocked.get_mut(row * 2 + i, col * 2 + j).unwrap();
                    *item = array.get(row, col).unwrap().get(i, j).cloned().unwrap();
                }
            }
        }
    }

    deblocked
}

#[cfg(test)]
mod tests {
    use array2::Array2;
    use crate::codec::trim_image;

    #[test]
    fn trim_array2() {
        let array = Array2::from_row_major(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        let array_trimmed = trim_image(array);
        assert_eq!(array_trimmed.iter_row_major().collect::<Vec<i32>>(), vec![1, 2, 4, 5]);
    }

    #[test]
    fn dont_trim_array2() {
        let array = Array2::from_row_major(vec![1, 2, 3, 4, 5, 6, 7, 8], 4);
        let array_trimmed = trim_image(array);
        assert_eq!(array_trimmed.iter_row_major().collect::<Vec<i32>>(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
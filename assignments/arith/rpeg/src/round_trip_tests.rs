// Round trip tests:
// Each test performs a set of compression steps on an image, then undoes those steps in reverse
// order. Each test adds a new step to the process. The resulting images are intended to be tested
// using `ppmdiff`.

#[cfg(test)]
mod tests {
    use array2::Array2;
    use csc411_image::{Read, RgbImage, Write};
    use crate::bitpack::{pack, unpack};
    use crate::codec::{blockify, deblockify, trim_image};
    use crate::normalize::{denormalize, normalize};
    use crate::component::{rgb_to_component, component_to_rgb, Component};
    use crate::dct::{CosineBlock, to_cosine_space, to_pixel_space};
    use crate::quantize::{dequantize, quantize, QuantizedBlock};

    #[test]
    fn _normalize() {
        let input = RgbImage::read(Some("img/enigma.ppm")).unwrap();
        let denominator = input.denominator;
        let image = Array2::from_row_major(input.pixels, input.width as usize);

        let trimmed = trim_image(image);

        let normalized = Array2::from_row_major(
            trimmed.iter_row_major()
                .map(|rgb| normalize(rgb, denominator))
                .collect(),
            trimmed.width);

        let denormalized = Array2::from_row_major(
            normalized.iter_row_major()
                .map(|normal_rgb| denormalize(normal_rgb))
                .collect(),
            normalized.width,
        );

        let output = RgbImage {
            pixels: denormalized.iter_row_major().collect(),
            width: denormalized.width as u32,
            height: denormalized.height as u32,
            denominator: 255,
        };

        output.write(Some("img/test_output/normalize.ppm")).expect("Error: normalize");
    }

    #[test]
    fn _component() {
        let input = RgbImage::read(Some("img/enigma.ppm")).unwrap();
        let denominator = input.denominator;
        let image = Array2::from_row_major(input.pixels, input.width as usize);

        let trimmed = trim_image(image);

        let normalized = Array2::from_row_major(
            trimmed.iter_row_major()
                .map(|rgb| normalize(rgb, denominator))
                .collect(),
            trimmed.width);

        let componentized = Array2::from_row_major(
            normalized.iter_row_major()
                .map(|normal| rgb_to_component(normal))
                .collect(),
            normalized.width,
        );

        let decomponentized = Array2::from_row_major(
            componentized.iter_row_major()
                .map(|component| component_to_rgb(component))
                .collect(),
            componentized.width,
        );

        let denormalized = Array2::from_row_major(
            decomponentized.iter_row_major()
                .map(|normal_rgb| denormalize(normal_rgb))
                .collect(),
            decomponentized.width,
        );

        let output = RgbImage {
            pixels: denormalized.iter_row_major().collect(),
            width: denormalized.width as u32,
            height: denormalized.height as u32,
            denominator: 255,
        };

        output.write(Some("img/test_output/component.ppm")).expect("Error: component");
    }

    #[test]
    fn _dct() {
        let input = RgbImage::read(Some("img/enigma.ppm")).unwrap();
        let denominator = input.denominator;
        let image = Array2::from_row_major(input.pixels, input.width as usize);

        let trimmed = trim_image(image);

        let normalized = Array2::from_row_major(
            trimmed.iter_row_major()
                .map(|rgb| normalize(rgb, denominator))
                .collect(),
            trimmed.width);

        let componentized = Array2::from_row_major(
            normalized.iter_row_major()
                .map(|normal| rgb_to_component(normal))
                .collect(),
            normalized.width,
        );

        let blocks = blockify(componentized, Component { y: 0.0, pb: 0.0, pr: 0.0 });

        let cosine_blocks: Vec<CosineBlock> = blocks.iter_row_major()
            .map(|block| to_cosine_space(block.clone()))
            .collect();

        let pixel_blocks: Array2<Array2<Component>> =
            Array2::from_row_major(
                cosine_blocks.iter()
                    .map(|block| to_pixel_space(block.clone()))
                    .collect(),
                blocks.width,
            );

        let deblocked = deblockify(pixel_blocks, Component { y: 0.0, pb: 0.0, pr: 0.0 });

        let decomponentized = Array2::from_row_major(
            deblocked.iter_row_major()
                .map(|component| component_to_rgb(component))
                .collect(),
            deblocked.width,
        );

        let denormalized = Array2::from_row_major(
            decomponentized.iter_row_major()
                .map(|normal_rgb| denormalize(normal_rgb))
                .collect(),
            decomponentized.width,
        );

        let output = RgbImage {
            pixels: denormalized.iter_row_major().collect(),
            width: denormalized.width as u32,
            height: denormalized.height as u32,
            denominator: 255,
        };

        output.write(Some("img/test_output/dct.ppm")).expect("Error: dct");
    }

    #[test]
    fn _quantize() {
        let input = RgbImage::read(Some("img/enigma.ppm")).unwrap();
        let denominator = input.denominator;
        let image = Array2::from_row_major(input.pixels, input.width as usize);

        let trimmed = trim_image(image);

        let normalized = Array2::from_row_major(
            trimmed.iter_row_major()
                .map(|rgb| normalize(rgb, denominator))
                .collect(),
            trimmed.width);

        let componentized = Array2::from_row_major(
            normalized.iter_row_major()
                .map(|normal| rgb_to_component(normal))
                .collect(),
            normalized.width,
        );

        let blocks = blockify(componentized, Component { y: 0.0, pb: 0.0, pr: 0.0 });

        let cosine_blocks: Vec<CosineBlock> = blocks.iter_row_major()
            .map(|block| to_cosine_space(block.clone()))
            .collect();

        let quantized: Vec<QuantizedBlock> = cosine_blocks.iter()
            .map(|block| quantize(block.clone()))
            .collect();

        let dequantized: Vec<CosineBlock> = quantized.iter()
            .map(|block| dequantize(block.clone()))
            .collect();

        let pixel_blocks: Array2<Array2<Component>> =
            Array2::from_row_major(
                dequantized.iter()
                    .map(|block| to_pixel_space(block.clone()))
                    .collect(),
                blocks.width,
            );

        let deblocked = deblockify(pixel_blocks, Component { y: 0.0, pb: 0.0, pr: 0.0 });

        let decomponentized = Array2::from_row_major(
            deblocked.iter_row_major()
                .map(|component| component_to_rgb(component))
                .collect(),
            deblocked.width,
        );

        let denormalized = Array2::from_row_major(
            decomponentized.iter_row_major()
                .map(|normal_rgb| denormalize(normal_rgb))
                .collect(),
            decomponentized.width,
        );

        let output = RgbImage {
            pixels: denormalized.iter_row_major().collect(),
            width: denormalized.width as u32,
            height: denormalized.height as u32,
            denominator: 255,
        };

        output.write(Some("img/test_output/quantize.ppm")).expect("Error: quantize");
    }

    #[test]
    fn _bitpack() {
        let input = RgbImage::read(Some("img/enigma.ppm")).unwrap();
        let denominator = input.denominator;
        let image = Array2::from_row_major(input.pixels, input.width as usize);

        let trimmed = trim_image(image);

        let normalized = Array2::from_row_major(
            trimmed.iter_row_major()
                .map(|rgb| normalize(rgb, denominator))
                .collect(),
            trimmed.width);

        let componentized = Array2::from_row_major(
            normalized.iter_row_major()
                .map(|normal| rgb_to_component(normal))
                .collect(),
            normalized.width,
        );

        let blocks = blockify(componentized, Component { y: 0.0, pb: 0.0, pr: 0.0 });

        let cosine_blocks: Vec<CosineBlock> = blocks.iter_row_major()
            .map(|block| to_cosine_space(block.clone()))
            .collect();

        let quantized: Vec<QuantizedBlock> = cosine_blocks.iter()
            .map(|block| quantize(block.clone()))
            .collect();

        let packed_words: Vec<u64> = quantized.iter()
            .map(|block| pack(block.clone()))
            .collect();

        let unpacked_blocks: Vec<QuantizedBlock> = packed_words.iter()
            .map(|word| unpack(word.clone()))
            .collect();

        let dequantized: Vec<CosineBlock> = unpacked_blocks.iter()
            .map(|block| dequantize(block.clone()))
            .collect();

        let pixel_blocks: Array2<Array2<Component>> =
            Array2::from_row_major(
                dequantized.iter()
                    .map(|block| to_pixel_space(block.clone()))
                    .collect(),
                blocks.width,
            );

        let deblocked = deblockify(pixel_blocks, Component { y: 0.0, pb: 0.0, pr: 0.0 });

        let decomponentized = Array2::from_row_major(
            deblocked.iter_row_major()
                .map(|component| component_to_rgb(component))
                .collect(),
            deblocked.width,
        );

        let denormalized = Array2::from_row_major(
            decomponentized.iter_row_major()
                .map(|normal_rgb| denormalize(normal_rgb))
                .collect(),
            decomponentized.width,
        );

        let output = RgbImage {
            pixels: denormalized.iter_row_major().collect(),
            width: denormalized.width as u32,
            height: denormalized.height as u32,
            denominator: 255,
        };

        output.write(Some("img/test_output/bitpack.ppm")).expect("Error: bitpack");
    }
}

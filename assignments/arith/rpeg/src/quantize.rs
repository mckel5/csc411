use crate::dct::CosineBlock;
use csc411_arith::{chroma_of_index, index_of_chroma};

/// A quantized version of a cosine-space block.
#[derive(Clone)]
pub struct QuantizedBlock {
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub pb_index: u64,
    pub pr_index: u64,
}

/// Describes the number of bits allocated to each discrete cosine value.
/// Used during the bitpacking process.
pub enum BitWidths {}

impl BitWidths {
    pub const A: u64 = 9;
    pub const B: u64 = 5;
    pub const C: u64 = 5;
    pub const D: u64 = 5;
    pub const PB_INDEX: u64 = 4;
    pub const PR_INDEX: u64 = 4;
}

static CLAMP_MIN: f32 = -0.3;
static CLAMP_MAX: f32 = 0.3;
static COSINE_SCALAR: f32 = ((2 << BitWidths::B - 2) as f32 - 1_f32) / CLAMP_MAX;

/// Quantize (i.e. scale to integer values) a cosine block of pixels.
///
/// Ranges:
/// * a: [0, 2^9 - 1]
/// * b: [-2^4 + 1, 2^4 - 1]
/// * c: [-2^4 + 1, 2^4 - 1]
/// * d: [-2^4 + 1, 2^4 - 1]
/// * Pb_index: [0, 2^4 - 1]
/// * Pr_index: [0, 2^4 - 1]
///
/// # Arguments:
/// * `cosine_block`: a set of cosine space values
pub fn quantize(cosine_block: CosineBlock) -> QuantizedBlock {
    QuantizedBlock {
        a: (cosine_block.a * (2_f32.powi(BitWidths::A as i32) - 1.0)).round() as u64,
        b: (cosine_block.b.clamp(CLAMP_MIN, CLAMP_MAX) * COSINE_SCALAR).round() as i64,
        c: (cosine_block.c.clamp(CLAMP_MIN, CLAMP_MAX) * COSINE_SCALAR).round() as i64,
        d: (cosine_block.d.clamp(CLAMP_MIN, CLAMP_MAX) * COSINE_SCALAR).round() as i64,
        pb_index: index_of_chroma(cosine_block.pb_avg) as u64,
        pr_index: index_of_chroma(cosine_block.pr_avg) as u64,
    }
}

/// Scale a quantized block back to real cosine values.
///
/// # Arguments:
/// * `quantized_block`: a set of quantized integer values
pub fn dequantize(quantized_block: QuantizedBlock) -> CosineBlock {
    CosineBlock {
        a: quantized_block.a as f32 / (2_f32.powi(BitWidths::A as i32) - 1.0),
        b: quantized_block.b as f32 / COSINE_SCALAR,
        c: quantized_block.c as f32 / COSINE_SCALAR,
        d: quantized_block.d as f32 / COSINE_SCALAR,
        pb_avg: chroma_of_index(quantized_block.pb_index as usize),
        pr_avg: chroma_of_index(quantized_block.pr_index as usize),
    }
}

#[cfg(test)]
mod tests {
    use csc411_arith::{chroma_of_index, index_of_chroma};
    use crate::dct::CosineBlock;
    use crate::quantize::{dequantize, quantize, QuantizedBlock};

    #[test]
    fn _quantize() {
        let input = CosineBlock {
            a: 0.55,
            b: -0.2,
            c: 0.3,
            d: -0.5,
            pb_avg: -0.05,
            pr_avg: 0.15,
        };

        let expected_output = QuantizedBlock {
            a: (input.a * 511_f32).round() as u64,
            b: (input.b.clamp(-0.3, 0.3) * 50_f32).round() as i64,
            c: (input.c.clamp(-0.3, 0.3) * 50_f32).round() as i64,
            d: (input.d.clamp(-0.3, 0.3) * 50_f32).round() as i64,
            pb_index: index_of_chroma(input.pb_avg) as u64,
            pr_index: index_of_chroma(input.pr_avg) as u64,
        };

        let output = quantize(input);

        assert_eq!(expected_output.a, output.a);
        assert_eq!(expected_output.b, output.b);
        assert_eq!(expected_output.c, output.c);
        assert_eq!(expected_output.d, output.d);
        assert_eq!(expected_output.pb_index, output.pb_index);
        assert_eq!(expected_output.pr_index, output.pr_index);
    }

    #[test]
    fn _dequantize() {
        let input = QuantizedBlock {
            a: 281,
            b: -10,
            c: 15,
            d: -15,
            pb_index: 1,
            pr_index: 9,
        };

        let expected_output = CosineBlock {
            a: input.a as f32 / 511.0,
            b: input.b as f32 / 50.0,
            c: input.c as f32 / 50.0,
            d: input.d as f32 / 50.0,
            pb_avg: chroma_of_index(input.pb_index as usize),
            pr_avg: chroma_of_index(input.pr_index as usize),
        };

        let output = dequantize(input);

        assert_float_absolute_eq!(expected_output.a, output.a);
        assert_float_absolute_eq!(expected_output.b, output.b);
        assert_float_absolute_eq!(expected_output.c, output.c);
        assert_float_absolute_eq!(expected_output.d, output.d);
        assert_float_absolute_eq!(expected_output.pb_avg, output.pb_avg);
        assert_float_absolute_eq!(expected_output.pr_avg, output.pr_avg);
    }
}
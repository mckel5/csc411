use bitpack::bitpack;
use crate::quantize::{QuantizedBlock, BitWidths};

// We only use 32 bits for our packed words
static USED_BITS: u64 = 32;

/// Pack a set of quantized values into a 64-bit word.
/// The leading 32 bits are unused, but a `u64` is used for compatibility with the `bitpack` module.
///
/// Bit allocation:
/// * a: 9 bits
/// * b: 5 bits
/// * c: 5 bits
/// * d: 5 bits
/// * Pb_avg: 4 bits
/// * Pr_avg: 4 bits
///
/// # Arguments:
/// * `quantized_block`: a set of quantized integer values
pub fn pack(quantized_block: QuantizedBlock) -> u64 {
    let QuantizedBlock { a, b, c, d, pb_index, pr_index } = quantized_block;
    let mut word = 0_u64;
    let mut position = 0_u64;

    word = bitpack::newu(word, BitWidths::PR_INDEX, position, pr_index).unwrap();
    position += BitWidths::PR_INDEX;
    word = bitpack::newu(word, BitWidths::PB_INDEX, position, pb_index).unwrap();
    position += BitWidths::PB_INDEX;
    word = bitpack::news(word, BitWidths::D, position, d).unwrap();
    position += BitWidths::D;
    word = bitpack::news(word, BitWidths::C, position, c).unwrap();
    position += BitWidths::C;
    word = bitpack::news(word, BitWidths::B, position, b).unwrap();
    position += BitWidths::B;
    word = bitpack::newu(word, BitWidths::A, position, a).unwrap();

    word
}

/// Unpack quantized values from a 64-bit word.
/// Only the rightmost 32 bits are unpacked.
///
/// # Arguments
/// * `word`: a packed 64-bit integer
pub fn unpack(word: u64) -> QuantizedBlock {
    let mut position = USED_BITS;

    position -= BitWidths::A;
    let a = bitpack::getu(word, BitWidths::A, position);
    position -= BitWidths::B;
    let b = bitpack::gets(word, BitWidths::B, position);
    position -= BitWidths::C;
    let c = bitpack::gets(word, BitWidths::C, position);
    position -= BitWidths::D;
    let d = bitpack::gets(word, BitWidths::D, position);
    position -= BitWidths::PB_INDEX;
    let pb_index = bitpack::getu(word, BitWidths::PB_INDEX, position);
    position -= BitWidths::PR_INDEX;
    let pr_index = bitpack::getu(word, BitWidths::PR_INDEX, position);

    QuantizedBlock { a, b, c, d, pb_index, pr_index }
}

#[cfg(test)]
mod tests {
    use crate::bitpack::{pack, unpack};
    use crate::quantize::QuantizedBlock;

    #[test]
    fn _pack() {
        let input = QuantizedBlock {
            a: 281,
            b: -10,
            c: 15,
            d: -15,
            pb_index: 1,
            pr_index: 9,
        };

        let expected: u64 = 0b100011001_10110_01111_10001_0001_1001;
        let packed = pack(input);
        assert_eq!(packed, expected);
    }

    #[test]
    fn _unpack() {
        let input: u64 = 0b100011001_10110_01111_10001_0001_1001;

        let expected = QuantizedBlock {
            a: 281,
            b: -10,
            c: 15,
            d: -15,
            pb_index: 1,
            pr_index: 9,
        };

        let unpacked = unpack(input);

        assert_eq!(unpacked.a, expected.a);
        assert_eq!(unpacked.b, expected.b);
        assert_eq!(unpacked.c, expected.c);
        assert_eq!(unpacked.d, expected.d);
        assert_eq!(unpacked.pb_index, expected.pb_index);
        assert_eq!(unpacked.pr_index, expected.pr_index);
    }
}
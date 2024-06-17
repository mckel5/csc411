static WORD_SIZE: u64 = 64;

/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 { return false; }
    if width == 1 { return n == 0 || n == -1; }
    if width >= 64 { return true; }
    let lower = -(2_i64 << (width - 2));
    let upper = (2_i64 << (width - 2)) - 1;
    lower <= n && n <= upper
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 { return false; }
    if width >= 64 { return true; }
    n < (2_u64 << (width - 1))
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    assert!(width + lsb <= WORD_SIZE, "Invalid width and LSB combination.");

    // Fields of width 0 always store 0
    if width == 0 { return 0; }

    // Fill LHS with zeroes or ones depending on leading bit
    let magnitude = WORD_SIZE - (lsb + width);
    let lhs_removed = arithmetic_shift_right(shift_left(word, magnitude) as i64, magnitude);
    // Shift value all the way to the right
    arithmetic_shift_right(lhs_removed, lsb)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    assert!(width + lsb <= WORD_SIZE, "Invalid width and LSB combination.");

    // Fields of width 0 always store 0
    if width == 0 { return 0; }

    // Zero out LHS
    let magnitude = WORD_SIZE - (lsb + width);
    let lhs_removed = logical_shift_right(shift_left(word, magnitude), magnitude);
    // Shift value all the way to the right
    logical_shift_right(lhs_removed, lsb)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(value, width) { return None; }

    // Isolate bits to left of value
    let lhs_magnitude = width + lsb;
    let lhs = shift_left(logical_shift_right(word.clone(), lhs_magnitude), lhs_magnitude);

    // Isolate bits to right of value
    let rhs_magnitude = WORD_SIZE - lsb;
    let rhs = logical_shift_right(shift_left(word.clone(), rhs_magnitude), rhs_magnitude);

    // Left-pad value as appropriate
    let value_shifted = shift_left(value, lsb);

    // Combine into new word
    Some(lhs | value_shifted | rhs)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width) { return None; }
    // We can use `newu` but we need to remove any leading 1s produced by 2's complement
    let magnitude = WORD_SIZE - width;
    let adjusted_value = logical_shift_right(shift_left(value as u64, magnitude), magnitude);
    newu(word, width, lsb, adjusted_value)
}

/// Bitwise shift a value left.
///
/// # Arguments
/// * `word`: the word to shift
/// * `magnitude`: the number of bits to shift by.
/// Returns all zeroes if this number is greater than or equal to the word width.
#[inline]
fn shift_left(word: u64, magnitude: u64) -> u64 {
    if magnitude >= WORD_SIZE { return 0_u64; }
    word << magnitude
}

/// Bitwise shift a value right logically.
///
/// # Arguments
/// * `word`: the word to shift
/// * `magnitude`: the number of bits to shift by.
/// Returns all zeroes if this number is greater than or equal to the word width.
#[inline]
fn logical_shift_right(word: u64, magnitude: u64) -> u64 {
    if magnitude >= WORD_SIZE { return 0_u64; }
    word >> magnitude
}

/// Bitwise shift a value right arithmetically.
/// Only used with 2's complement signed integers.
///
/// # Arguments
/// * `word`: the word to shift
/// * `magnitude`: the number of bits to shift by.
/// Fills the entire word with the first bit of the word if this number is greater than or equal to the word width.
#[inline]
fn arithmetic_shift_right(word: i64, magnitude: u64) -> i64 {
    if magnitude >= WORD_SIZE {
        // 2's complement: copy first bit over entire word
        return if word < 0 { i64::MIN } else { 0_i64 };
    }
    word >> magnitude
}

#[cfg(test)]
mod tests {
    use crate::bitpack::{arithmetic_shift_right, fitss, fitsu, gets, getu, logical_shift_right, news, newu, shift_left};

    #[test]
    fn _shift_left() {
        assert_eq!(shift_left(0b11, 2), 0b1100);
        // "Out of bounds" shift
        assert_eq!(shift_left(0b1010, 64), 0);
        assert_eq!(shift_left(0b1010, 1000), 0);
    }

    #[test]
    fn _logical_shift_right() {
        assert_eq!(logical_shift_right(0b1100, 2), 0b11);
        // "Out of bounds" shift
        assert_eq!(logical_shift_right(0b1010, 64), 0);
        assert_eq!(logical_shift_right(0b1010, 1000), 0);
    }

    #[test]
    fn _arithmetic_shift_right() {
        assert_eq!(arithmetic_shift_right(0b1100, 2), 0b11);
        // "Out of bounds" shift
        assert_eq!(arithmetic_shift_right(50, 64), 0);
        assert_eq!(arithmetic_shift_right(-50, 64), i64::MIN);
    }

    #[test]
    fn _fitsu() {
        assert!(fitsu(31, 5));
        assert!(!fitsu(32, 5));
    }

    #[test]
    fn _fitss() {
        assert!(fitss(15, 4));
        assert!(!fitss(16, 4));
        assert!(fitss(-16, 4));
    }

    #[test]
    fn _get() {
        // Example bitpacking scenario
        //                 a         b     c     d     pbi  pri
        let word: u64 = 0b_100101101_00100_10001_00000_1111_0101;
        assert_eq!(getu(word, 4, 0), 5);
        assert_eq!(getu(word, 4, 4), 15);
        assert_eq!(gets(word, 5, 8), 0);
        assert_eq!(gets(word, 5, 13), -15);
        assert_eq!(gets(word, 5, 18), 4);
        assert_eq!(getu(word, 9, 23), 301);

        // Positive signed and unsigned integers are handled the same
        assert_eq!(getu(0b0111, 2, 2), gets(0b0111, 2, 2) as u64);
        // Negative signed integers are handled differently than positive integers
        assert_ne!(getu(0b1111, 2, 2) as i64, gets(0b1111, 2, 2));
    }

    #[test]
    fn _new() {
        assert_eq!(newu(0b00000000, 3, 2, 0b111), Some(0b00011100));
        assert_eq!(newu(0b11111111, 4, 3, 0b1111), Some(0b11111111));
        assert_eq!(newu(0b11111111, 4, 3, 0b1), Some(0b10001111));
        // Value does not fit
        assert_eq!(newu(0b0000, 2, 0, 0b1111), None);
        // Positive integers handled the same
        assert_eq!(newu(0b00000000, 3, 2, 0b111), news(0b00000000, 3, 2, 0b111));

        // Appropriate handling of 2s complement integers
        assert_eq!(news(0, 3, 0, 0b101_i64), Some(0b101_u64));
    }
}

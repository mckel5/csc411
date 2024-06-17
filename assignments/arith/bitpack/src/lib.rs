pub mod bitpack;

// SIZE = width of word
// f    = (w, lsb)
// f_h  = (SIZE - (lsb + w), lsb + w)
// f_l  = (lsb, 0)

#[cfg(test)]
mod tests {
    use crate::bitpack;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        for w in 0..65 {
            for lsb in 0..(65 - w) {
                for _ in 0..1001 {
                    let word = rng.gen_range(0..u64::MAX);
                    let value = rng.gen_range(0..u64::MAX);
                    let w2 = rng.gen_range(0..65);
                    let lsb2 = rng.gen_range(0..(65 - w2));
                    check_laws(word, w, lsb, value, w2, lsb2);
                }
            }
        }
    }

    fn check_laws(word: u64, w: u64, lsb: u64, value: u64, w2: u64, lsb2: u64) {
        // Simply reinterprets the bits of the u64 as an i64
        let signed_value = value as i64;

        // Check if `fits` functions work properly
        assert_eq!(bitpack::fitsu(value, w), value <= (2_u128.pow(w as u32) - 1) as u64);
        if w > 0 {
            assert_eq!(bitpack::fitss(signed_value, w),
                       (-2_i128.pow(w as u32 - 1)) as i64 <= signed_value && signed_value <= (2_i128.pow(w as u32 - 1) - 1) as i64
            );
        }

        // If `bitpack::news` is given a `value` that does not fit in `width` signed bits, it must return `None`.
        if !bitpack::fitss(signed_value, w) {
            assert_eq!(bitpack::news(word, w, lsb, signed_value), None);
            return; // Later tests will necessarily fail, so return early
        }

        // If `bitpack::newu` is given a `value` that does not fit in `width` unsigned bits, it must return `None`.
        if !bitpack::fitsu(value, w) {
            assert_eq!(bitpack::newu(word, w, lsb, value), None);
            return; // Later tests will necessarily fail, so return early
        }

        assert_eq!(
            bitpack::getu(bitpack::newu(word, w, lsb, value).unwrap(), w, lsb),
            value
        );

        if lsb2 >= w + lsb {
            assert_eq!(
                bitpack::getu(bitpack::newu(word, w, lsb, value).unwrap(), w2, lsb2),
                bitpack::getu(word, w2, lsb2)
            );
        }

        assert_eq!(
            bitpack::gets(bitpack::news(word, w, lsb, signed_value).unwrap(), w, lsb),
            signed_value
        );

        if lsb2 >= w + lsb {
            assert_eq!(
                bitpack::gets(bitpack::news(word, w, lsb, signed_value).unwrap(), w2, lsb2),
                bitpack::gets(word, w2, lsb2)
            );
        }
    }
}
#[derive(Default, Debug)]
/// A Universal Machine instruction.
/// Not all fields will be used for a given instruction.
pub struct Instruction {
    pub opcode: u32,
    pub reg_a: u32,
    pub reg_b: u32,
    pub reg_c: u32,
    pub reg_load: u32,
    pub load_value: u32,
}

const WORD_SIZE: u8 = 32;

/// Disassemble a binary word into a Universal Machine instruction.
///
/// # Arguments
/// - `word`: a `u32` word
///
/// # Returns
/// An `Instruction`
pub fn disassemble(word: &u32) -> Instruction {
    Instruction {
        opcode: get_bits(word, 28, 4),
        reg_a: get_bits(word, 6, 3),
        reg_b: get_bits(word, 3, 3),
        reg_c: get_bits(word, 0, 3),
        reg_load: get_bits(word, 25, 3),
        load_value: get_bits(word, 0, 25),
    }
}

fn get_bits(word: &u32, lsb: u8, width: u8) -> u32 {
    // Fields of width 0 always store 0
    if width == 0 {
        return 0;
    }

    // Zero out LHS
    let magnitude = WORD_SIZE - (lsb + width);
    let lhs_removed = (word << magnitude) >> magnitude;

    // Shift value all the way to the right
    lhs_removed >> lsb
}

#[cfg(test)]
mod tests {
    use super::disassemble;

    #[test]
    fn segmented_load() {
        //                op   unused              rA  rB  rC
        let word: u32 = 0b0001_0000000000000000000_110_010_101;
        let product = &disassemble(&word);

        assert_eq!(product.opcode, 1);
        assert_eq!(product.reg_a, 6);
        assert_eq!(product.reg_b, 2);
        assert_eq!(product.reg_c, 5);
    }

    #[test]
    fn load_value() {
        //                op   rL  value
        let word: u32 = 0b1101_111_1001101001010100101010001;
        let product = &disassemble(&word);

        assert_eq!(product.opcode, 13);
        assert_eq!(product.reg_load, 7);
        assert_eq!(product.load_value, 20228433);
    }
}

use std::fs::File;
use std::io::{BufReader, Read};

/// Load the input file (`.um` or `.umz`) into memory.
///
/// # Arguments
/// - `f`: the `File` object
///
/// # Returns
/// A `Vec` of `u32` words
///
/// # Panics
/// - If the number of bytes in `f` is not divisible by 4
pub fn load(f: File) -> Vec<u32> {
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector
    let _ = reader.read_to_end(&mut buffer);
    assert_eq!(
        buffer.len() % 4,
        0,
        "Input file must be a series of 32-bit words."
    );

    // Convert u8's to u32's (values remain unchanged)
    let u32_buffer: Vec<u32> = buffer.iter().map(|byte| byte.to_owned() as u32).collect();
    // Split into chunks of 4 bytes
    let chunked_bytes = u32_buffer.chunks(4).collect::<Vec<_>>();
    // Combine each chunk into a single word
    chunked_bytes
        .iter()
        .map(|chunk| chunk[0] << 24 | chunk[1] << 16 | chunk[2] << 8 | chunk[3])
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{remove_file, File},
        io::Write,
    };

    use super::load;

    #[test]
    fn load_word() {
        let path = "/tmp/rumload_test";

        let mut f = File::create(path).unwrap();
        let buffer: [u8; 4] = [0b00110011, 0b10000001, 0b11110000, 0b00100101];
        f.write_all(&buffer).unwrap();

        let f = File::open(path).unwrap();
        let word = load(f)[0];

        remove_file(path).unwrap();

        assert_eq!(word, 0b00110011100000011111000000100101);
        // Sanity check               v
        assert_ne!(word, 0b00110011100100011111000000100101);
    }
}

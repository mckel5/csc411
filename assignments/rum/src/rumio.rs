use std::io::{stdin, stdout, Read, Write};

/// Send a single byte to the output device.
pub fn output(value: u8) {
    let mut stdout = stdout().lock();
    let _ = stdout.write_all(&value.to_be_bytes());
    let _ = stdout.flush();
}

/// Get a single byte from the input device.
pub fn input() -> u8 {
    stdin().bytes().next().unwrap().unwrap()
}

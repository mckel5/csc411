use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::load;
use rumasm::rumasm::loadv;
use rumasm::rumasm::map;
use rumasm::rumasm::output;
use rumasm::rumasm::store;
use rumasm::rumasm::unmap;

pub fn main() {
    let s = String::from("hello!");

    // New segment length
    asm(loadv(0, s.len() as u32));

    // Map segment, storing index in register 0
    asm(map(0, 0));

    // Store each value into memory
    for (i, char) in s.chars().enumerate() {
        asm(loadv(1, i as u32));
        asm(loadv(2, char as u32));
        asm(store(0, 1, 2));
    }

    // Load each value from memory, then output
    for i in 0..s.len() {
        asm(loadv(1, i as u32));
        asm(load(2, 0, 1));
        asm(output(2));
    }

    // Free memory
    asm(unmap(0));
    asm(halt());
}

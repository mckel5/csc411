use rand::{self, Rng};
use std::{collections::BTreeMap, process::exit};

pub type Memory = BTreeMap<u32, Vec<u32>>;

/// Load a program into memory segment 0.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `program`: a `Vec` of UM words
pub fn load_program(memory: &mut Memory, program: Vec<u32>) {
    memory.insert(0, program);
}

/// Get the length of the currently loaded program.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
///
/// # Returns
/// The length of the program
pub fn get_program_length(memory: &Memory) -> usize {
    memory[&0].len()
}

/// Map a memory segment.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `length`: the length (in words) of the new segment
///
/// # Returns
/// The index of the new memory segment
pub fn map(memory: &mut Memory, length: u32) -> u32 {
    if memory.keys().len() == u32::MAX as usize {
        eprintln!("Out of memory!");
        exit(1);
    }
    let segment = vec![0_u32; length as usize];
    let index = choose_open_index(memory);
    memory.insert(index, segment);
    index
}

/// Unmap a memory segment. Does nothing if the specified segment is not mapped.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `index`: index of segment to remove
pub fn unmap(memory: &mut Memory, index: u32) {
    memory.remove(&index);
}

/// Load a value from memory.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `segment_index`: the index of the segment to be loaded from
/// - `offset`: the position within the segment to be loaded from
///
/// # Returns
/// The value at the specified memory location
///
/// # Panics
/// - If `segment_index` refers to an unmapped segment
/// - If `offset` is greater than the length of the segment
pub fn load(memory: &Memory, segment_index: u32, offset: u32) -> u32 {
    memory[&segment_index][offset as usize]
}

/// Store a value in memory.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `value`: the value to store
/// - `segment_index`: the index of the segment to be loaded from
/// - `offset`: the position within the segment to be loaded from
///
/// # Panics
/// - If `segment_index` refers to an unmapped segment
/// - If `offset` is greater than the length of the segment
pub fn store(memory: &mut Memory, value: u32, segment_index: u32, offset: u32) {
    memory.get_mut(&segment_index).unwrap()[offset as usize] = value;
}

/// Choose an index for mapping a new segment.
fn choose_open_index(memory: &Memory) -> u32 {
    let mut index = 0;
    let mut rng = rand::thread_rng();

    while memory.contains_key(&index) {
        index = rng.gen();
    }

    index
}
// fn choose_open_index(memory: &Memory) -> u32 {
//     let mut index = 0;

//     while memory.contains_key(&index) {
//         index += 1;
//     }

//     index
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_segment() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);
        assert_eq!(memory[&index].len(), 10);
    }

    #[test]
    fn unmap_segment() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);
        assert_eq!(memory.contains_key(&index), true);
        unmap(&mut memory, index);
        assert_eq!(memory.contains_key(&index), false);
    }

    #[test]
    fn default_value() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);

        for i in 0..10 {
            assert_eq!(memory[&index][i], 0);
        }
    }

    #[test]
    fn load_store() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);
        let value = 59;
        let offset = 3;
        store(&mut memory, value, index, offset);
        let loaded_value = load(&memory, index, offset);
        assert_eq!(value, loaded_value);
    }
}

// pub type Memory = Vec<Vec<u32>>;

use std::ops::{Index, IndexMut};

type Segment = Vec<u32>;

/// A representation of the UM's virtual, segmented memory.
pub struct Memory {
    memory: Vec<Segment>,
    free_segments: Vec<usize>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: vec![vec![]; 1],
            free_segments: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn push(&mut self, value: Segment) {
        self.memory.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&Segment> {
        self.memory.get(index)
    }
}

impl Index<usize> for Memory {
    type Output = Segment;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory.get(index).unwrap()
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.memory.get_mut(index).unwrap()
    }
}

/// Load a program into memory segment 0.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `program`: a `Vec` of UM words
pub fn load_program(memory: &mut Memory, program: Vec<u32>) {
    memory[0] = program;
}

/// Get the length of the currently loaded program.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
///
/// # Returns
/// The length of the program
pub fn get_program_length(memory: &Memory) -> usize {
    memory[0].len()
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
    match memory.free_segments.pop() {
        Some(index) => {
            memory[index] = vec![0; length as usize];
            return index as u32;
        }
        None => {
            memory.push(vec![0; length as usize]);
            return (memory.len() - 1) as u32;
        }
    }
}

/// Unmap a memory segment. Does nothing if the specified segment is not mapped.
///
/// # Arguments
/// - `memory`: the segmented memory of the UM
/// - `index`: index of segment to remove
pub fn unmap(memory: &mut Memory, index: u32) {
    memory.free_segments.push(index as usize)
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
    memory[segment_index as usize][offset as usize]
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
    memory[segment_index as usize][offset as usize] = value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_segment() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);
        assert_eq!(memory[index as usize].len(), 10);
    }

    #[test]
    fn unmap_segment() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);
        assert_eq!(memory.get(index as usize), Some(vec![0; 10].as_ref()));
        assert_eq!(memory.free_segments.len(), 0);
        unmap(&mut memory, index);
        assert_eq!(memory.free_segments.len(), 1);
        assert_eq!(memory.free_segments[0], 1);
        map(&mut memory, 50);
        assert_eq!(memory.get(index as usize), Some(vec![0; 50].as_ref()));
    }

    #[test]
    fn default_value() {
        let mut memory = Memory::new();
        let index = map(&mut memory, 10);

        for i in 0..10 {
            assert_eq!(memory[index as usize][i], 0);
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

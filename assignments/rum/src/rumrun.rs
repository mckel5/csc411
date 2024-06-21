use crate::{
    rumdis, rumio,
    rummem::{self, get_program_length},
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

type Registers = [u32; 8];

#[derive(FromPrimitive)]
#[repr(u32)]
enum Operation {
    ConditionalMove,
    LoadSegment,
    StoreSegment,
    Add,
    Multiply,
    Divide,
    NAND,
    Halt,
    Map,
    Unmap,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}

pub fn execute(mut memory: &mut rummem::Memory) {
    let mut program_counter = 0_usize;
    let mut program_length = rummem::get_program_length(&memory);
    let mut registers: Registers = [0; 8];

    while program_counter != program_length {
        // Get next instruction
        let rumdis::Instruction {
            opcode,
            reg_a,
            reg_b,
            reg_c,
            reg_load,
            load_value,
        } = rumdis::disassemble(&memory[&0][program_counter as usize]);

        // match FromPrimitive::from_u32(opcode) {
        //     Some(Operation::LoadValue) => {
        //         println!(
        //             "[{}] op: {}, ra: {}, rb: {}, rc: {}, val: {}",
        //             program_counter, opcode, reg_load, 0, 0, load_value
        //         );
        //     }
        //     _ => {
        //         println!(
        //             "[{}] op: {}, ra: {}, rb: {}, rc: {}, val: {}",
        //             program_counter, opcode, reg_a, reg_b, reg_c, 0
        //         );
        //     }
        // }

        // Execute instruction
        match FromPrimitive::from_u32(opcode) {
            Some(Operation::ConditionalMove) => {
                if registers[reg_c as usize] != 0 {
                    registers[reg_a as usize] = registers[reg_b as usize];
                }
            }
            Some(Operation::LoadSegment) => {
                registers[reg_a as usize] =
                    memory[&registers[reg_b as usize]][registers[reg_c as usize] as usize];
            }
            Some(Operation::StoreSegment) => {
                memory.get_mut(&registers[reg_a as usize]).unwrap()
                    [registers[reg_b as usize] as usize] = registers[reg_c as usize];
            }
            Some(Operation::Add) => {
                registers[reg_a as usize] =
                    u32::wrapping_add(registers[reg_b as usize], registers[reg_c as usize]);
            }
            Some(Operation::Multiply) => {
                registers[reg_a as usize] =
                    u32::wrapping_mul(registers[reg_b as usize], registers[reg_c as usize]);
            }
            Some(Operation::Divide) => {
                registers[reg_a as usize] = registers[reg_b as usize] / registers[reg_c as usize];
            }
            Some(Operation::NAND) => {
                registers[reg_a as usize] =
                    !(registers[reg_b as usize] & registers[reg_c as usize]);
            }
            Some(Operation::Halt) => {
                return;
            }
            Some(Operation::Map) => {
                registers[reg_b as usize] = rummem::map(&mut memory, registers[reg_c as usize]);
            }
            Some(Operation::Unmap) => {
                rummem::unmap(&mut memory, registers[reg_c as usize]);
            }
            Some(Operation::Output) => {
                rumio::output(registers[reg_c as usize] as u8);
            }
            Some(Operation::Input) => {
                registers[reg_c as usize] = rumio::input() as u32;
            }
            Some(Operation::LoadProgram) => {
                let program = memory[&registers[reg_b as usize]].clone();
                rummem::load_program(&mut memory, program);
                program_counter = registers[reg_c as usize] as usize;
                program_length = get_program_length(memory);
                // Avoid incrementing the program counter
                continue;
            }
            Some(Operation::LoadValue) => {
                registers[reg_load as usize] = load_value;
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }

        program_counter += 1;
    }
}

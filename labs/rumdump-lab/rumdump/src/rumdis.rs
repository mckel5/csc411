use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
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

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}

pub fn disassemble(inst: Umi) -> String {
    match op(inst) {
        Some(Opcode::ConditionalMove) => {
            format!(
                "if (r{} != 0) then r{} := r{}",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::LoadSegment) => {
            format!(
                "r{} := $m[r{}][r{}]",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::StoreSegment) => {
            format!(
                "$m[r{}][r{}] := r{}",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Add) => {
            format!(
                "r{} := (r{} + r{}) % 2^32",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Multiply) => {
            format!(
                "r{} := (r{} * r{}) % 2^32",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Divide) => {
            format!(
                "r{} := (r{} / r{})",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::NAND) => {
            format!(
                "r{} := !(r{} & r{})",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Halt) => String::from("Halt."),
        Some(Opcode::Map) => {
            format!(
                "r{} loaded with the index of a free memory segment. New segment created at $m[r{}] with length = r{} words and value = 0.",
                get(&RB, inst),
                get(&RB, inst),
                get(&RC, inst),
            )
        }
        Some(Opcode::Unmap) => {
            format!("Memory segment $m[r{}] unmapped.", get(&RC, inst))
        }
        Some(Opcode::Output) => {
            format!(
                "Value in r{} displayed on I/O device.",
                get(&RC, inst),
            )
        }
        Some(Opcode::Input) => {
            format!(
                "Loading I/O input into r{}.",
                get(&RC, inst),
            )
        }
        Some(Opcode::LoadProgram) => {
            format!(
                "Segment $m[r{}] copied into $m[0]. Program counter set to r{}.",
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::LoadValue) => {
            format!(
                "r{} := {}",
                get(&RL, inst),
                get(&VL, inst)
            )
        }
        _ => format!("Invalid instruction {}", inst),
    }
}

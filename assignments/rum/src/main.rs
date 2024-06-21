use std::{env, fs::File};

use rum::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert_eq!(argnum, 2, "Usage: rum <file.um/file.umz>");
    let file = File::open(args.iter().nth(1).unwrap()).unwrap();

    let program = rumload::load(file);
    let mut memory = rummem::Memory::new();
    rummem::load_program(&mut memory, program);

    rumrun::execute(&mut memory);
}

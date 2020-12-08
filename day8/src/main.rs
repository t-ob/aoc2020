use std::io::{self, Read};

use day8::{Instruction, Machine, Program};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Part 1
    let program: Program = buffer
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();

    let mut machine = Machine::new();
    machine.load(&program);

    let _ = machine.run();

    println!("{}", machine.acc());

    // Part 2
    for (idx, instruction) in program.iter().enumerate().skip(1) {
        let new_instruction: Instruction;
        match instruction {
            Instruction::Jmp(v) => new_instruction = Instruction::Nop(*v),
            Instruction::Nop(v) => new_instruction = Instruction::Jmp(*v),
            _ => continue,
        }
        let mut new_program = program.clone();
        if let Some(instruction) = new_program.get_mut(idx) {
            *instruction = new_instruction;
        }
        machine.load(&new_program);
        if let Ok(acc) = machine.run() {
            println!("{}", acc);
        }
    }

    Ok(())
}

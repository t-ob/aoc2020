use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.trim().split_ascii_whitespace().collect();
        match tokens[..] {
            ["nop", nop] => {
                if let Ok(v) = nop.parse::<i64>() {
                    Ok(Instruction::Nop(v))
                } else {
                    Err(format!("Unable to parse int: {}", nop))
                }
            },
            ["acc", val] => {
                if let Ok(v) = val.parse::<i64>() {
                    Ok(Instruction::Acc(v))
                } else {
                    Err(format!("Unable to parse int: {}", val))
                }
            }
            ["jmp", offset] => {
                if let Ok(offset) = offset.parse::<i64>() {
                    Ok(Instruction::Jmp(offset))
                } else {
                    Err(format!("Unable to parse int: {}", offset))
                }
            }
            _ => Err(format!("Invalid instruction: {}", s)),
        }
    }
}

pub type Program = Vec<Instruction>;

#[derive(Debug, Eq, PartialEq)]
pub enum MachineError {
    ProgramNotLoaded,
    OutOfBounds,
    InfiniteLoop
}

pub struct Machine {
    pc: i64,
    acc: i64,
    program: Option<Program>,
}

impl Machine {
    pub fn new() -> Machine {
        let pc = 0;
        let acc = 0;
        let program = None;

        Machine { pc, acc, program }
    }

    pub fn acc(&self) -> i64 {
        self.acc
    }

    pub fn pc(&self) -> i64 {
        self.pc
    }

    pub fn load(&mut self, program: &Program) {
        self.pc = 0;
        self.acc = 0;
        self.program = Some(program.clone())
    }

    pub fn run(&mut self) -> Result<i64, MachineError> {
        self.pc = 0;
        self.acc = 0;
        if let Some(program) = &self.program {
            let mut seen: HashSet<i64> = HashSet::new();

            loop {
                if seen.contains(&self.pc) {
                    return Err(MachineError::InfiniteLoop)
                }
                seen.insert(self.pc);
                if let Some(instruction) = program.get(self.pc as usize) {
                    match instruction {
                        Instruction::Acc(v) => {
                            self.acc += v;
                            self.pc += 1;
                        }
                        Instruction::Jmp(offset) => {
                            let pc = self.pc + *offset;
                            if pc < 0 || pc as usize > program.len() {
                                return Err(MachineError::OutOfBounds);
                            } else {
                                self.pc = pc;
                            }
                        }
                        Instruction::Nop(_) => self.pc += 1,
                    }
                } else {
                    break
                }
            }
        } else {
            return Err(MachineError::ProgramNotLoaded)
        }

        Ok(self.acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!("nop +0".parse::<Instruction>(), Ok(Instruction::Nop(0)));
        assert_eq!("acc +1".parse::<Instruction>(), Ok(Instruction::Acc(1)));
        assert_eq!("jmp +4".parse::<Instruction>(), Ok(Instruction::Jmp(4)));
        assert_eq!("acc -99".parse::<Instruction>(), Ok(Instruction::Acc(-99)));
        assert_eq!("jmp -3".parse::<Instruction>(), Ok(Instruction::Jmp(-3)));
    }

    #[test]
    fn test_run_without_program() {
        let mut machine = Machine::new();
        assert_eq!(machine.run(), Err(MachineError::ProgramNotLoaded))
    }

    #[test]
    fn test_infinite_loop() {
        let program: Program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];

        let mut machine = Machine::new();
        machine.load(&program);

        assert_eq!(machine.run(), Err(MachineError::InfiniteLoop))
    }

    #[test]
    fn test_ok() {
        let program: Program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Nop(-4),
            Instruction::Acc(6),
        ];

        let mut machine = Machine::new();
        machine.load(&program);

        assert_eq!(machine.run(), Ok(8))
    }
}

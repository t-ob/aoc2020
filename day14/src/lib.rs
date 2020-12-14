#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

pub enum Version {
    V1,
    V2,
}

pub enum Instruction {
    Mask(Vec<char>),
    SetMemory(usize, u64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
        }
        lazy_static! {
            static ref SET_MEMORY_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }

        if let Some(captures) = MASK_RE.captures(s) {
            return Ok(Instruction::Mask(captures[1].chars().collect()));
        }

        if let Some(captures) = SET_MEMORY_RE.captures(s) {
            return Ok(Instruction::SetMemory(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
            ));
        }

        Err(format!("Unable to decode instruction: {}", s))
    }
}

pub type Program = Vec<Instruction>;

pub struct Machine {
    version: Version,
    floating_mask: usize,
    and_mask: u64,
    or_mask: u64,
    memory: HashMap<usize, u64>, // a 36-bit address space of u64s would require 8 * 64GB of available memory, so using a hash map and hoping for the best.
}

impl Machine {
    pub fn new(version: Version) -> Machine {
        let floating_mask = 0;
        let and_mask = !0;
        let or_mask = 0;
        let memory = HashMap::new();

        Machine {
            version,
            floating_mask,
            and_mask,
            or_mask,
            memory,
        }
    }

    fn set_mask(&mut self, pattern: &[char]) -> Result<(), String> {
        if pattern.len() != 36 {
            return Err(format!(
                "Invalid mask length. Got {}, expected 36",
                pattern.len()
            ));
        }
        let mut floating_mask = 0;
        let mut and_mask = 0;
        let mut or_mask = 0;

        for c in pattern {
            floating_mask <<= 1;
            and_mask <<= 1;
            or_mask <<= 1;
            match c {
                '0' => {}
                '1' => or_mask |= 1,
                'X' => {
                    and_mask |= 1;
                    floating_mask |= 1
                }
                _ => return Err(format!("Invalid mask character: {}", c)),
            }
        }

        self.floating_mask = floating_mask;

        self.and_mask = and_mask;
        self.or_mask = or_mask;

        Ok(())
    }

    fn set_memory(&mut self, address: usize, value: u64) {
        match self.version {
            Version::V1 => {
                let value = (value & self.and_mask) | self.or_mask;

                self.memory.insert(address, value);
            }
            Version::V2 => {
                let floating_mask = self.floating_mask;
                let or_mask = self.or_mask as usize;
                let unfloated_address = address & !floating_mask | or_mask;

                let ones = floating_mask.count_ones() as usize;
                let mut floating_mask_powers_of_two: Vec<usize> = Vec::with_capacity(ones);
                for bit_position in 0..36 {
                    let power_of_two = 1 << bit_position;
                    if floating_mask & power_of_two > 0 {
                        floating_mask_powers_of_two.push(power_of_two);
                    }
                }
                for floating_mask_idx in 0..(1 << ones) {
                    let mut floating_mask = 0;
                    for power_of_two_idx in 0..ones {
                        if floating_mask_idx & (1 << power_of_two_idx) > 0 {
                            floating_mask |= floating_mask_powers_of_two[power_of_two_idx];
                        }
                    }

                    self.memory.insert(floating_mask | unfloated_address, value);
                }
            }
        }
    }

    fn reset(&mut self) {
        self.memory = HashMap::new();
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        self.reset();
        for instruction in program {
            match instruction {
                Instruction::Mask(mask) => self.set_mask(mask)?,
                Instruction::SetMemory(addr, val) => self.set_memory(*addr, *val),
            }
        }

        Ok(())
    }

    pub fn check(&self) -> u64 {
        self.memory.values().map(|x| *x).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_v1() {
        let mut machine = Machine::new(Version::V1);

        let program: Program = vec![
            Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".chars().collect()),
            Instruction::SetMemory(8, 11),
            Instruction::SetMemory(7, 101),
            Instruction::SetMemory(8, 0),
        ];

        let _ = machine.run(&program);

        assert_eq!(machine.check(), 165)
    }

    #[test]
    fn test_machine_v2() {
        let mut machine = Machine::new(Version::V2);

        let program: Program = vec![
            Instruction::Mask("000000000000000000000000000000X1001X".chars().collect()),
            Instruction::SetMemory(42, 100),
            Instruction::Mask("00000000000000000000000000000000X0XX".chars().collect()),
            Instruction::SetMemory(26, 1),
        ];

        let _ = machine.run(&program);

        assert_eq!(machine.check(), 208)
    }
}

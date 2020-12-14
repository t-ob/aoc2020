use std::io::{self, Read};

use day14::{Machine, Program, Version};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let program: Program = buffer.lines().map(|line| line.parse().unwrap()).collect();

    // Part 1
    let mut machine = Machine::new(Version::V1);

    let _ = machine.run(&program);

    println!("{}", machine.check());

    // Part 2
    let mut machine_2 = Machine::new(Version::V2);

    let _ = machine_2.run(&program);

    println!("{}", machine_2.check());

    Ok(())
}

use std::io::{self, Read};

use day15::MemoryGame;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    let mut memory_game_1 = MemoryGame::new(&input);

    println!("{}", memory_game_1.nth(2020 - 1).unwrap());

    // Part 2
    println!("{}", memory_game_1.nth(30000000 - 2020 - 1).unwrap());

    Ok(())
}

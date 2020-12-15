use std::io::{self, Read};

use day15::MemoryGame;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    let mut memory_game_1 = MemoryGame::new(&input);

    println!("{}", memory_game_1.nth(2020 - 1).unwrap());

    // Part 2
    let mut memory_game_2 = MemoryGame::new(&input);

    println!("{}", memory_game_2.nth(30000000 - 1).unwrap());

    Ok(())
}

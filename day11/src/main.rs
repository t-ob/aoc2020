use std::io::{self, Read};

use day11::{Automata, Mode, Tile};

fn main() -> io::Result<()> {
    let mut buffer_1 = String::new();
    io::stdin().read_to_string(&mut buffer_1)?;

    let buffer_2 = buffer_1.clone();

    let automata_1 = buffer_1.parse::<Automata>().unwrap();
    let mut automata_2 = buffer_2.parse::<Automata>().unwrap();
    automata_2.set_mode(Mode::Directional);

    for mut automata in vec![automata_1, automata_2] {
        let mut tiles = automata.next().unwrap();
        loop {
            let next_tiles = automata.next().unwrap();
            if tiles == next_tiles {
                break;
            }
            tiles = next_tiles;
        }
        let occupied_seats = tiles
            .iter()
            .map(|row| row.iter().filter(|t| **t == Tile::OccupiedSeat).count())
            .fold(0, |x, y| x + y);

        println!("{}", occupied_seats);
    }

    Ok(())
}

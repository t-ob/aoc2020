use std::io::{self, Read};

use day3::{Tile, TobogganMap};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let toboggan_map: TobogganMap = buffer.parse().unwrap();

    // Part 1
    let mut trees_encountered = 0;
    let (mut x, mut y) = (0, 0);

    while y < toboggan_map.rows() {
        if toboggan_map.get(x, y) == Tile::Tree {
            trees_encountered += 1;
        }
        x += 3;
        y += 1;
    }

    println!("{}", trees_encountered);

    // Part 2
    let mut trees_encountered_product: u32 = 1;
    for (dx, dy) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut trees_encountered = 0;
        let (mut x, mut y) = (0, 0);

        while y < toboggan_map.rows() {
            if toboggan_map.get(x, y) == Tile::Tree {
                trees_encountered += 1;
            }
            x += dx;
            y += dy;
        }

        trees_encountered_product *= trees_encountered;
    }

    println!("{}", trees_encountered_product);

    Ok(())
}

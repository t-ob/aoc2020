use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use day24::CyclotomicInteger;

static EAST: CyclotomicInteger = CyclotomicInteger(1, 0);
static NORTH_EAST: CyclotomicInteger = CyclotomicInteger(0, 1);
static NORTH_WEST: CyclotomicInteger = CyclotomicInteger(-1, 1);
static WEST: CyclotomicInteger = CyclotomicInteger(-1, 0);
static SOUTH_WEST: CyclotomicInteger = CyclotomicInteger(0, -1);
static SOUTH_EAST: CyclotomicInteger = CyclotomicInteger(1, -1);

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut destination_tiles: Vec<CyclotomicInteger> = Vec::new();
    let mut destination_counts = HashMap::new();

    for line in buffer.lines() {
        let mut steps: Vec<CyclotomicInteger> = Vec::new();
        let mut cs = line.trim().chars();
        loop {
            match cs.next() {
                None => break,
                Some('e') => steps.push(EAST),
                Some('w') => steps.push(WEST),
                Some('n') => match cs.next() {
                    Some('e') => steps.push(NORTH_EAST),
                    Some('w') => steps.push(NORTH_WEST),
                    _ => panic!("Invalid input"),
                },
                Some('s') => match cs.next() {
                    Some('e') => steps.push(SOUTH_EAST),
                    Some('w') => steps.push(SOUTH_WEST),
                    _ => panic!("Invalid input"),
                },
                _ => panic!("Invalid input"),
            }
        }

        let destination = steps.iter().fold(CyclotomicInteger(0, 0), |a, s| a + *s);
        destination_tiles.push(destination);

        let e = destination_counts.entry(destination).or_insert(0);
        *e += 1;
    }

    let mut black_tiles = destination_counts
        .iter()
        .filter(|(_, c)| *c & 1 == 1)
        .map(|(t, _)| *t)
        .collect::<HashSet<_>>();

    // Part 1
    println!("{}", black_tiles.len());

    // Part 2
    let mut i = 0;
    while i < 100 {
        let mut to_consider = HashMap::new();
        for z in black_tiles.iter() {
            if !to_consider.contains_key(z) {
                to_consider.insert(*z, HashSet::new());
            }

            for dz in [EAST, NORTH_EAST, NORTH_WEST, WEST, SOUTH_WEST, SOUTH_EAST].iter() {
                let neighbouring_black_tiles =
                    to_consider.entry(*z + *dz).or_insert(HashSet::new());
                neighbouring_black_tiles.insert(*z);
            }
        }

        black_tiles = to_consider
            .keys()
            .filter(
                |z| match (black_tiles.contains(z), &to_consider.get(z).unwrap().len()) {
                    (true, 1) | (true, 2) => true,
                    (false, 2) => true,
                    _ => false,
                },
            )
            .map(|z| *z)
            .collect();

        i += 1;
    }

    println!("{}", black_tiles.len());

    Ok(())
}

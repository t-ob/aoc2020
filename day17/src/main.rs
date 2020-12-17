use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

static DELTAS_3: [(i64, i64, i64); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

static DELTAS_4: [(i64, i64, i64, i64); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut part_1: HashSet<(i64, i64, i64)> = buffer
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i64, y as i64, 0))
        })
        .flatten()
        .collect();

    let mut part_2: HashSet<(i64, i64, i64, i64)> = buffer
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i64, y as i64, 0, 0))
        })
        .flatten()
        .collect();

    let mut turn = 0;

    while turn < 6 {
        // Part 1
        let mut to_consider_part_1: HashMap<(i64, i64, i64), HashSet<(i64, i64, i64)>> =
            HashMap::new();

        part_1
            .iter()
            .map(|(x, y, z)| {
                DELTAS_3
                    .iter()
                    .map(move |(dx, dy, dz)| ((*x + *dx, *y + *dy, *z + *dz), (*x, *y, *z)))
            })
            .flatten()
            .for_each(|(p, q)| {
                let neighbours = to_consider_part_1.entry(p).or_insert(HashSet::new());
                (*neighbours).insert(q);
            });

        part_1 = to_consider_part_1
            .keys()
            .filter(|k| {
                let neighbour_count = &to_consider_part_1.get(k).unwrap().len();
                match (part_1.contains(k), neighbour_count) {
                    (true, 2) => true,
                    (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                }
            })
            .cloned()
            .collect();

        // Part 2
        let mut to_consider_part_2: HashMap<(i64, i64, i64, i64), HashSet<(i64, i64, i64, i64)>> =
            HashMap::new();

        part_2
            .iter()
            .map(|(x, y, z, w)| {
                DELTAS_4.iter().map(move |(dx, dy, dz, dw)| {
                    ((*x + *dx, *y + *dy, *z + *dz, *w + *dw), (*x, *y, *z, *w))
                })
            })
            .flatten()
            .for_each(|(p, q)| {
                let neighbours = to_consider_part_2.entry(p).or_insert(HashSet::new());
                (*neighbours).insert(q);
            });

        part_2 = to_consider_part_2
            .keys()
            .filter(|k| {
                let neighbour_count = &to_consider_part_2.get(k).unwrap().len();
                match (part_2.contains(k), neighbour_count) {
                    (true, 2) => true,
                    (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                }
            })
            .cloned()
            .collect();

        turn += 1;
    }

    println!("{:?}", part_1.len());
    println!("{:?}", part_2.len());

    Ok(())
}

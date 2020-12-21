use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use rand::seq::SliceRandom;
use rand::Rng; // 0.7.2

use day20::{Picture, Tile10, Tile8, D4};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let symmetries = buffer
        .split("\n\n")
        .filter_map(|s| s.parse::<Tile10>().ok())
        .map(|tile| tile.symmetries())
        .collect::<Vec<_>>();

    let mut neighbours_to_left = HashMap::new();
    let mut neighbours_to_up = HashMap::new();

    for (orientation, neighbour_map) in [1, 2]
        .iter()
        .zip([&mut neighbours_to_left, &mut neighbours_to_up].iter_mut())
    {
        symmetries
            .iter()
            .enumerate()
            .map(|(i, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .map(move |(j, tile)| (tile.signature()[*orientation], (i, j)))
            })
            .flatten()
            .for_each(|(signature, xy)| {
                let neighbours = neighbour_map.entry(signature).or_insert_with(HashSet::new);
                neighbours.insert(xy);
            })
    }

    let path = (0..12)
        .map(|x| (0..12).map(move |y| (x, y)))
        .flatten()
        .collect::<Vec<_>>();

    let mut grid: [[Option<Tile10>; 12]; 12] = [[None; 12]; 12];
    let mut found = false;
    let mut rng = rand::thread_rng();
    while !found {
        let mut random_walk_grid: [[Option<Tile10>; 12]; 12] = [[None; 12]; 12];
        let mut random_walk_path = path.clone();
        let mut random_walk_seen = HashSet::new();
        while !random_walk_path.is_empty() {
            match random_walk_path.pop().unwrap() {
                (11, 11) => {
                    let random_walk_start = symmetries[rng.gen_range(0..144)][rng.gen_range(0..8)];
                    random_walk_seen.insert(random_walk_start.id());
                    random_walk_grid[11][11] = Some(random_walk_start);
                }
                (11, col) => {
                    let random_walk_candidates_from_right = neighbours_to_left
                        .get(&random_walk_grid[11][col + 1].unwrap().signature()[3]);
                    match random_walk_candidates_from_right {
                        Some(random_walk_candidates_to_right) => {
                            let filtered_random_walk_candidates = random_walk_candidates_to_right
                                .iter()
                                .map(|(i, j)| symmetries[*i][*j])
                                .filter(|tile| !random_walk_seen.contains(&tile.id()))
                                .collect::<Vec<_>>();

                            if let Some(next_tile) =
                                filtered_random_walk_candidates.choose(&mut rng)
                            {
                                random_walk_seen.insert(next_tile.id());
                                random_walk_grid[11][col] = Some(*next_tile);
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                (row, 11) => {
                    let random_walk_candidates_from_down = neighbours_to_up
                        .get(&random_walk_grid[row + 1][11].unwrap().signature()[0]);
                    match random_walk_candidates_from_down {
                        Some(random_walk_candidates_from_down) => {
                            let filtered_random_walk_candidates = random_walk_candidates_from_down
                                .iter()
                                .map(|(i, j)| symmetries[*i][*j])
                                .filter(|tile| !random_walk_seen.contains(&tile.id()))
                                .collect::<Vec<_>>();

                            if let Some(next_tile) =
                                filtered_random_walk_candidates.choose(&mut rng)
                            {
                                random_walk_seen.insert(next_tile.id());
                                random_walk_grid[row][11] = Some(*next_tile);
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                (row, col) => {
                    let random_walk_candidates_from_down = neighbours_to_up
                        .get(&random_walk_grid[row + 1][col].unwrap().signature()[0]);
                    let random_walk_candidates_from_right = neighbours_to_left
                        .get(&random_walk_grid[row][col + 1].unwrap().signature()[3]);

                    match (
                        random_walk_candidates_from_down,
                        random_walk_candidates_from_right,
                    ) {
                        (
                            Some(random_walk_candidates_from_down),
                            Some(random_walk_candidates_from_right),
                        ) => {
                            let filtered_random_walk_candidates = random_walk_candidates_from_down
                                .intersection(random_walk_candidates_from_right)
                                .map(|(i, j)| symmetries[*i][*j])
                                .filter(|tile| !random_walk_seen.contains(&tile.id()))
                                .collect::<Vec<_>>();

                            if let Some(next_tile) =
                                filtered_random_walk_candidates.choose(&mut rng)
                            {
                                random_walk_seen.insert(next_tile.id());
                                random_walk_grid[row][col] = Some(*next_tile);
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            }
        }

        if random_walk_path.is_empty() {
            found = true;
            grid = random_walk_grid;
        }
    }

    println!(
        "{:?}",
        grid[0][0].unwrap().id()
            * grid[0][11].unwrap().id()
            * grid[11][0].unwrap().id()
            * grid[11][11].unwrap().id()
    );

    let mut inner_grid: [[Tile8; 12]; 12] = [[Tile8::new(); 12]; 12];
    for i in 0..12 {
        for j in 0..12 {
            inner_grid[i][j] = grid[i][j].unwrap().inner();
        }
    }

    let picture = Picture::from_tile_8_arrays(inner_grid);

    println!(
        "{}",
        picture
            .symmetries()
            .iter()
            .map(|sym| sym.choppiness())
            .min()
            .unwrap()
    );

    Ok(())
}

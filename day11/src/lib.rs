use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug)]
pub enum Mode {
    Adjacent,
    Directional,
}

#[derive(Debug)]
pub struct Automata {
    tiles: Vec<Vec<Tile>>,
    mode: Mode,
}

impl Automata {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Automata {
        let mode = Mode::Adjacent;
        Automata { tiles, mode }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

fn part_1(tiles: &Vec<Vec<Tile>>, p: (usize, usize)) -> Tile {
    let (x, y) = p;
    let deltas = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let adjacent_occupied = deltas
        .iter()
        .map(|(dx, dy)| (x as i64 + dx, y as i64 + dy))
        .filter(|(x, y)| *x >= 0 && *x < tiles[0].len() as i64 && *y >= 0 && *y < tiles.len() as i64)
        .map(|(x, y)| tiles[y as usize][x as usize])
        .filter(|tile| *tile == Tile::OccupiedSeat)
        .count();

    let tile = tiles[y][x];
    match (tile, adjacent_occupied) {
        (Tile::OccupiedSeat, x) if x >= 4 => Tile::EmptySeat,
        (Tile::EmptySeat, 0) => Tile::OccupiedSeat,
        _ => tile,
    }
}

fn part_2(tiles: &Vec<Vec<Tile>>, p: (usize, usize)) -> Tile {
    let (x, y) = p;
    let deltas = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut nearby_occupied = 0;
    for (dx, dy) in deltas {
        let mut x = x as i64 + dx;
        let mut y = y as i64 + dy;
        while x >= 0 && x < tiles[0].len() as i64 && y >= 0 && y < tiles.len() as i64 {
            let tile = tiles[y as usize][x as usize];
            match tile {
                Tile::OccupiedSeat => {
                    nearby_occupied += 1;
                    break;
                }
                Tile::EmptySeat => break,
                Tile::Floor => {
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    let tile = tiles[y][x];
    match (tile, nearby_occupied) {
        (Tile::OccupiedSeat, x) if x >= 5 => Tile::EmptySeat,
        (Tile::EmptySeat, 0) => Tile::OccupiedSeat,
        _ => tile,
    }
}

impl Iterator for Automata {
    type Item = Vec<Vec<Tile>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.tiles.clone();

        let idxs = (0..self.tiles.len()).map(|y| (0..self.tiles[0].len()).map(move |x| (x, y)));

        let new = idxs
            .map(|i| {
                i.map(|(x, y)| match self.mode {
                    Mode::Adjacent => part_1(&self.tiles, (x, y)),
                    Mode::Directional => part_2(&self.tiles, (x, y)),
                })
                .collect()
            })
            .collect();

        self.tiles = new;

        Some(current)
    }
}

impl FromStr for Automata {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for line in s.lines() {
            let mut row: Vec<Tile> = Vec::new();
            for char in line.trim().chars() {
                match char {
                    'L' => row.push(Tile::EmptySeat),
                    '#' => row.push(Tile::OccupiedSeat),
                    '.' => row.push(Tile::Floor),
                    _ => return Err(format!("Invalid character: {}", char)),
                }
            }
            tiles.push(row);
        }
        Ok(Automata::new(tiles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let f = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut automata: Automata = f.parse().unwrap();

        let expected_first = Some(vec![
            vec![
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
            vec![
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
            vec![
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::Floor,
            ],
            vec![
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
            vec![
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
            vec![
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
            vec![
                Tile::Floor,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
            ],
            vec![
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
            vec![
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
            ],
            vec![
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::EmptySeat,
                Tile::Floor,
                Tile::EmptySeat,
                Tile::EmptySeat,
            ],
        ]);

        assert_eq!(automata.next(), expected_first);

        let expected_second = Some(vec![
            vec![
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::Floor,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::Floor,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
                Tile::Floor,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
            ],
            vec![
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
                Tile::Floor,
                Tile::OccupiedSeat,
                Tile::OccupiedSeat,
            ],
        ]);

        assert_eq!(automata.next(), expected_second);
    }
}

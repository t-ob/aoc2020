use std::str::FromStr;

pub trait D4: Sized + Copy + Clone {
    fn symmetries(&self) -> [Self; 8] {
        let i = self.clone();
        [
            i,
            i.mirror(1),
            i.rotate(1),
            i.rotate(2),
            i.rotate(3),
            i.rotate(1).mirror(1),
            i.rotate(2).mirror(1),
            i.rotate(3).mirror(1),
        ]
    }
    fn mirror(&self, n: usize) -> Self;
    fn rotate(&self, n: usize) -> Self;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile10(usize, [u16; 10]);

impl D4 for Tile10 {
    fn rotate(&self, n: usize) -> Tile10 {
        let mut rows: [u16; 10] = self.1.clone();
        for _ in 0..(n % 4) {
            let current_rows = rows.clone();
            let cols = (0..10).map(|idx| {
                current_rows
                    .iter()
                    .rev()
                    .map(|x| (*x & (1 << (9 - idx))) >> (9 - idx))
                    .fold(0, |a, x| (a << 1) | x)
            });

            for (i, col) in cols.enumerate() {
                rows[i] = col;
            }
        }

        Tile10(self.0, rows)
    }

    fn mirror(&self, n: usize) -> Tile10 {
        let mut rows: [u16; 10] = self.1.clone();
        for _ in 0..(n % 2) {
            let current_rows = rows.clone();

            for (i, row) in current_rows.iter().rev().enumerate() {
                rows[i] = *row;
            }
        }

        Tile10(self.0, rows)
    }
}

impl Tile10 {
    pub fn signature(&self) -> [u16; 4] {
        let up = self.1[0];
        let right = self.1.iter().map(|x| *x & 0x1).fold(0, |a, x| (a << 1) | x);
        let down = self.1[9];
        let left = self
            .1
            .iter()
            .map(|x| (*x & 0x200) >> 9)
            .fold(0, |a, x| (a << 1) | x);

        [up, right, down, left]
    }

    pub fn id(&self) -> usize {
        self.0
    }

    pub fn inner(&self) -> Tile8 {
        let mut tile = [0; 8];
        for (idx, val) in self
            .1
            .iter()
            .skip(1)
            .take(8)
            .map(|x| (*x >> 1 & 0xFF) as u8)
            .enumerate()
        {
            tile[idx] = val;
        }
        Tile8(tile)
    }
}

impl FromStr for Tile10 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: [u16; 10] = [0; 10];
        let mut lines = s.lines();
        if let Some(header) = lines.next() {
            if let Some(id) = header.split(|x| x == ' ' || x == ':').skip(1).next() {
                if let Ok(id) = id.parse() {
                    let x = lines.map(|l| {
                        l.trim().chars().fold(Ok(0), |a, c| match (a, c) {
                            (Ok(a), '.') => Ok(a << 1),
                            (Ok(a), '#') => Ok((a << 1) + 1),
                            _ => Err(()),
                        })
                    });
                    for (i, b) in x.enumerate() {
                        rows[i] = b?
                    }

                    return Ok(Tile10(id, rows));
                }
            }
        }
        Err(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile8([u8; 8]);

impl Tile8 {
    pub fn new() -> Tile8 {
        Tile8([0; 8])
    }
}

static SEA_MONSTER: [u128; 3] = [
    0b00000000000000000010,
    0b10000110000110000111,
    0b01001001001001001000,
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Picture([u128; 96]);

impl Picture {
    pub fn new() -> Picture {
        Picture([0; 96])
    }

    pub fn from_tile_8_arrays(tiles: [[Tile8; 12]; 12]) -> Picture {
        let mut pixels = [0; 96];

        for i in 0..12 {
            for j in 0..12 {
                for k in 0..8 {
                    pixels[8 * i + k] |= (tiles[i][j].0[k] as u128) << ((11 - j) * 8);
                }
            }
        }

        Picture(pixels)
    }

    pub fn choppiness(&self) -> u32 {
        let mut ones = self.0.iter().map(|x| x.count_ones()).sum::<u32>();

        let sea_monster_ones = SEA_MONSTER.iter().map(|x| x.count_ones()).sum::<u32>();

        for i in 0..(96 - 3) {
            for j in 0..(96 - 20) {
                let r1 = SEA_MONSTER[0] & (self.0[i] >> j);
                let r2 = SEA_MONSTER[1] & (self.0[i + 1] >> j);
                let r3 = SEA_MONSTER[2] & (self.0[i + 2] >> j);

                if r1 == SEA_MONSTER[0] && r2 == SEA_MONSTER[1] && r3 == SEA_MONSTER[2] {
                    ones -= sea_monster_ones;
                }
            }
        }

        ones
    }
}

impl D4 for Picture {
    fn rotate(&self, n: usize) -> Picture {
        let mut rows: [u128; 96] = self.0.clone();

        for _ in 0..(n % 4) {
            let mut rotated: [u128; 96] = [0; 96];

            for i in 0..96 {
                for j in 0..96 {
                    rotated[i] |= (1 & (rows[95 - j] >> (95 - i))) << (95 - j);
                }
            }

            rows = rotated;
        }

        Picture(rows)
    }

    fn mirror(&self, n: usize) -> Picture {
        let mut rows: [u128; 96] = self.0.clone();
        for _ in 0..(n % 2) {
            let current_rows = rows.clone();

            for (i, row) in current_rows.iter().rev().enumerate() {
                rows[i] = *row;
            }
        }

        Picture(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let tile = Tile10(
            0,
            [
                0b0000011111,
                0b1100110101,
                0b0000001011,
                0b1000010101,
                0b1000000110,
                0b1000100101,
                0b1000000000,
                0b0000000110,
                0b1010010000,
                0b1010111001,
            ],
        );

        assert_eq!(
            tile.signature(),
            [0b0000011111, 0b1111010001, 0b1010111001, 0b0101111011]
        )
    }

    #[test]
    fn test_rotate() {
        let tile = Tile10(
            0,
            [
                0b0000011111,
                0b1100110101,
                0b0000001011,
                0b1000010101,
                0b1000000110,
                0b1000100101,
                0b1000000000,
                0b0000000110,
                0b1010010000,
                0b1010111001,
            ],
        );

        let expected_1 = Tile10(
            0,
            [
                0b1101111010,
                0b0000000010,
                0b1100000000,
                0b0000000000,
                0b1000100010,
                0b1100001011,
                0b1000000101,
                0b0010111011,
                0b0010010101,
                0b1000101111,
            ],
        );

        let expected_2 = Tile10(
            0,
            [
                0b1001110101,
                0b0000100101,
                0b0110000000,
                0b0000000001,
                0b1010010001,
                0b0110000001,
                0b1010100001,
                0b1101000000,
                0b1010110011,
                0b1111100000,
            ],
        );

        let expected_3 = Tile10(
            0,
            [
                0b1111010001,
                0b1010100100,
                0b1101110100,
                0b1010000001,
                0b1101000011,
                0b0100010001,
                0b0000000000,
                0b0000000011,
                0b0100000000,
                0b0101111011,
            ],
        );

        assert_eq!(tile.rotate(0), tile);
        assert_eq!(tile.rotate(1), expected_1);
        assert_eq!(tile.rotate(2), expected_2);
        assert_eq!(tile.rotate(3), expected_3);
    }

    #[test]
    fn test_mirror() {
        let tile = Tile10(
            0,
            [
                0b0000011111,
                0b1100110101,
                0b0000001011,
                0b1000010101,
                0b1000000110,
                0b1000100101,
                0b1000000000,
                0b0000000110,
                0b1010010000,
                0b1010111001,
            ],
        );

        let expected = Tile10(
            0,
            [
                0b1010111001,
                0b1010010000,
                0b0000000110,
                0b1000000000,
                0b1000100101,
                0b1000000110,
                0b1000010101,
                0b0000001011,
                0b1100110101,
                0b0000011111,
            ],
        );

        assert_eq!(tile.mirror(0), tile);
        assert_eq!(tile.mirror(1), expected);
    }

    #[test]
    fn test_inner() {
        let tile = Tile10(
            0,
            [
                0b0000011111,
                0b1100110101,
                0b0000001011,
                0b1000010101,
                0b1000000110,
                0b1000100101,
                0b1000000000,
                0b0000000110,
                0b1010010000,
                0b1010111001,
            ],
        );

        let expected = Tile8([
            0b10011010, 0b00000101, 0b00001010, 0b00000011, 0b00010010, 0b00000000, 0b00000011,
            0b01001000,
        ]);

        let expected_m = Tile8([
            0b01001000, 0b00000011, 0b00000000, 0b00010010, 0b00000011, 0b00001010, 0b00000101,
            0b10011010,
        ]);

        let expected_r1 = Tile8([
            0b00000001, 0b10000000, 0b00000000, 0b00010001, 0b10000101, 0b00000010, 0b01011101,
            0b01001010,
        ]);

        let expected_r2 = Tile8([
            0b00010010, 0b11000000, 0b00000000, 0b01001000, 0b11000000, 0b01010000, 0b10100000,
            0b01011001,
        ]);

        let expected_r3 = Tile8([
            0b01010010, 0b10111010, 0b01000000, 0b10100001, 0b10001000, 0b00000000, 0b00000001,
            0b10000000,
        ]);

        assert_eq!(tile.inner(), expected);
        assert_eq!(tile.rotate(1).inner(), expected_r1);
        assert_eq!(tile.rotate(2).inner(), expected_r2);
        assert_eq!(tile.rotate(3).inner(), expected_r3);
        assert_eq!(tile.mirror(1).inner(), expected_m);
    }

    #[test]
    fn test_grid_construction() {
        let mut tiles: [[Tile8; 12]; 12] = [[const_tile_8(0); 12]; 12];
        for i in 0..12 {
            for j in 0..12 {
                tiles[i][j] = const_tile_8(12 * i as u8 + j as u8)
            }
        }

        let grid = Picture::from_tile_8_arrays(tiles);

        for i in 0..96 {
            for j in 0..96 {
                let row = grid.0[i];
                let lhs = ((row >> (95 - j)) & 1) as u8;
                let tile8 = tiles[i / 8][j / 8];
                let rhs = tile8.0[i % 8] >> (7 - (j % 8)) & 1;

                assert_eq!(lhs, rhs);
            }
        }
    }

    fn const_tile_8(x: u8) -> Tile8 {
        Tile8([x, x, x, x, x, x, x, x])
    }

    #[test]
    fn test_grid_d4() {
        let mut tiles: [[Tile8; 12]; 12] = [[const_tile_8(0); 12]; 12];
        for i in 0..12 {
            for j in 0..12 {
                tiles[i][j] = const_tile_8(12 * i as u8 + j as u8)
            }
        }

        let grid = Picture::from_tile_8_arrays(tiles);

        assert_ne!(grid, grid.rotate(1));
        assert_ne!(grid, grid.rotate(2));
        assert_ne!(grid, grid.rotate(3));
        assert_eq!(grid, grid.rotate(4));
        assert_ne!(grid, grid.mirror(1));
        assert_eq!(grid, grid.mirror(2));

        assert_eq!(grid, grid.rotate(1).rotate(1).rotate(1).rotate(1));
        assert_eq!(grid, grid.mirror(1).mirror(1));
    }

    #[test]
    fn test_grid_d4_distinct() {
        let mut tiles: [[Tile8; 12]; 12] = [[const_tile_8(0); 12]; 12];
        for i in 0..12 {
            for j in 0..12 {
                tiles[i][j] = const_tile_8(12 * i as u8 + j as u8)
            }
        }

        let grid = Picture::from_tile_8_arrays(tiles);

        let syms = grid.symmetries();

        for i in 0..8 {
            for j in i..8 {
                if i == j {
                    continue;
                }
                assert_ne!(syms[i], syms[j])
            }
        }
    }

    #[test]
    fn test_choppiness() {
        let mut pixels: [u128; 96] = [0; 96];
        pixels[0] = SEA_MONSTER[0];
        pixels[1] = SEA_MONSTER[1];
        pixels[2] = SEA_MONSTER[2];
        let picture = Picture(pixels);

        assert_eq!(picture.choppiness(), 0);
    }
}

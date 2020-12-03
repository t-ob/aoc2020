use std::{error::Error, str::FromStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Open,
    Tree,
}

#[derive(Debug)]
pub struct ParseMapError {
    details: String,
}

impl std::fmt::Display for ParseMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseMapError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl ParseMapError {
    fn new(msg: &str) -> ParseMapError {
        ParseMapError {
            details: msg.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TobogganMap {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for TobogganMap {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for line in s.lines() {
            let row: Result<Vec<Tile>, Self::Err> = line
                .chars()
                .map(|char| match char {
                    '.' => Ok(Tile::Open),
                    '#' => Ok(Tile::Tree),
                    _ => Err(ParseMapError::new(&format!("Invalid character: {}", char))),
                })
                .collect();
            match row {
                Ok(tile) => tiles.push(tile),
                Err(err) => return Err(err),
            }
        }
        Ok(TobogganMap { tiles })
    }
}

impl TobogganMap {
    pub fn get(&self, x: usize, y: usize) -> Tile {
        let cols = self.tiles[0].len();
        self.tiles[y][x % cols]
    }

    pub fn rows(&self) -> usize {
        self.tiles.len()
    }
}

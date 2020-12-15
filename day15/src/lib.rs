use std::collections::{HashMap, VecDeque};

pub struct MemoryGame {
    turn: usize,
    seen: HashMap<i64, usize>,
    last_spoken: Option<i64>,
    next_spoken: Option<i64>,
    starting_numbers: VecDeque<i64>,
}

impl MemoryGame {
    pub fn new(starting_numbers: &[i64]) -> MemoryGame {
        let turn = 0;
        let seen = HashMap::new();
        let last_seen = None;
        let next_spoken = None;
        let starting_numbers = starting_numbers
            .iter()
            .map(|s| *s)
            .collect::<VecDeque<i64>>();
        MemoryGame {
            turn,
            seen,
            last_spoken: last_seen,
            next_spoken,
            starting_numbers,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last_spoken, self.starting_numbers.pop_front()) {
            (None, None) => return None,
            (None, Some(starting_number)) => {
                self.next_spoken = Some(starting_number);
            }
            (Some(last_spoken), Some(starting_number)) => {
                self.next_spoken = Some(starting_number);
                self.seen.insert(last_spoken, self.turn);
            }
            (Some(last_spoken), None) => {
                if let Some(prev_turn) = self.seen.get(&last_spoken) {
                    self.next_spoken = Some((self.turn - prev_turn) as i64);
                } else {
                    self.next_spoken = Some(0);
                }
                self.seen.insert(last_spoken, self.turn);
            }
        }

        self.last_spoken = self.next_spoken;

        self.turn += 1;

        return self.last_spoken;
    }
}

#[cfg(test)]
mod tests {
    use crate::MemoryGame;

    #[test]
    fn test_memory_game() {
        let mut memory_game = MemoryGame::new(&[0, 3, 6]);

        assert_eq!(memory_game.next(), Some(0));
        assert_eq!(memory_game.next(), Some(3));
        assert_eq!(memory_game.next(), Some(6));
        assert_eq!(memory_game.next(), Some(0));
        assert_eq!(memory_game.next(), Some(3));
        assert_eq!(memory_game.next(), Some(3));
        assert_eq!(memory_game.next(), Some(1));
        assert_eq!(memory_game.next(), Some(0));
        assert_eq!(memory_game.next(), Some(4));
        assert_eq!(memory_game.next(), Some(0));
    }

    #[test]
    fn test_memory_game_2020() {
        let mut memory_game = MemoryGame::new(&[1, 3, 2]);
        assert_eq!(memory_game.nth(2020 - 1), Some(1))
    }

    #[test]
    fn test_memory_game_30000000() {
        let mut memory_game = MemoryGame::new(&[1, 3, 2]);
        assert_eq!(memory_game.nth(30000000 - 1), Some(2578))
    }
}

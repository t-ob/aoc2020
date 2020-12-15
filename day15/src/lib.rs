use std::collections::VecDeque;

pub struct MemoryGame {
    turn: usize,
    seen: Vec<Option<usize>>,
    last_spoken: Option<usize>,
    next_spoken: Option<usize>,
    starting_numbers: VecDeque<usize>,
}

impl MemoryGame {
    pub fn new(starting_numbers: &[usize]) -> MemoryGame {
        let turn = 0;
        let seen = vec![None; 1 << 25];
        let last_spoken = None;
        let next_spoken = None;
        let starting_numbers = starting_numbers
            .iter()
            .map(|s| *s)
            .collect::<VecDeque<usize>>();
        MemoryGame {
            turn,
            seen,
            last_spoken,
            next_spoken,
            starting_numbers,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last_spoken, self.starting_numbers.pop_front()) {
            (None, None) => return None,
            (None, Some(starting_number)) => {
                self.next_spoken = Some(starting_number);
            }
            (Some(last_spoken), Some(starting_number)) => {
                self.next_spoken = Some(starting_number);

                self.seen[last_spoken] = Some(self.turn);
            }
            (Some(last_spoken), None) => {
                if let Some(prev_turn) = self.seen[last_spoken] {
                    self.next_spoken = Some(self.turn - prev_turn);
                } else {
                    self.next_spoken = Some(0);
                }
                self.seen[last_spoken] = Some(self.turn);
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

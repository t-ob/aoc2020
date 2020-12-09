use std::collections::{HashMap, HashSet, VecDeque};

pub struct Preamble {
    counter: HashMap<i64, usize>,
    sum_deque: VecDeque<(i64, HashSet<i64>)>,
}

impl Preamble {
    pub fn new(seed: &[i64]) -> Preamble {
        let len = seed.len();
        let mut counter: HashMap<_, _> = HashMap::new();
        let mut sum_deque: VecDeque<_> = VecDeque::new();
        for i in 0..=len - 1 {
            let mut sums: HashSet<_> = HashSet::new();
            for j in i + 1..=len - 1 {
                let x = seed[i] + seed[j];
                sums.insert(x);
                *counter.entry(x).or_insert(0) += 1;
            }
            sum_deque.push_back((seed[i], sums));
        }

        Preamble { counter, sum_deque }
    }

    pub fn is_valid(&self, x: i64) -> bool {
        self.counter.contains_key(&x)
    }

    pub fn push(&mut self, x: i64) {
        let (_, sums) = self.sum_deque.pop_front().unwrap();
        for sum in sums.iter() {
            let count = self.counter.get_mut(sum).unwrap();
            *count -= 1;
            if 0 == *count {
                self.counter.remove(sum);
            }
        }

        for (value, sums) in self.sum_deque.iter_mut() {
            let sum = *value + x;
            sums.insert(sum);
            *self.counter.entry(sum).or_insert(0) += 1;
        }

        self.sum_deque.push_back((x, HashSet::new()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preamble() {
        let seed = [
            20, 7, 8, 13, 6, 16, 15, 23, 21, 25, 18, 17, 12, 2, 14, 22, 9, 11, 4, 1, 3, 5, 19, 24,
            10,
        ];

        assert_eq!(seed.len(), 25);

        let mut preamble = Preamble::new(&seed);

        assert!(preamble.is_valid(26));
        assert!(preamble.is_valid(49));
        assert!(!preamble.is_valid(100));
        assert!(!preamble.is_valid(50));

        preamble.push(45);

        assert!(preamble.is_valid(26));
        assert!(!preamble.is_valid(65));
        assert!(preamble.is_valid(64));
        assert!(preamble.is_valid(66));
    }
}

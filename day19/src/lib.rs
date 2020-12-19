use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
enum Pattern {
    C(Vec<char>),
    O(Vec<usize>, Vec<usize>),
    A(Vec<usize>),
}

pub struct Matcher {
    patterns: HashMap<usize, Pattern>,
}

impl Matcher {
    pub fn matches(&self, input: &str) -> bool {
        match self.partial_matches(input, 0) {
            None => false,
            Some(deltas) => deltas.contains(&input.len()),
        }
    }

    fn partial_matches(&self, input: &str, from_pattern: usize) -> Option<Vec<usize>> {
        if input.is_empty() {
            return None;
        }
        match &self.patterns.get(&from_pattern).unwrap() {
            Pattern::C(chars) if input.chars().take(chars.len()).collect::<Vec<_>>() == *chars => {
                Some(vec![chars.len()])
            }
            Pattern::C(_) => None,
            Pattern::O(lhs, rhs) => {
                let mut lhs_found = false;
                let mut lhs_matched_idxs = vec![0];
                for idx in lhs {
                    let mut new_matched_idxs = vec![];

                    for matched_idx in &lhs_matched_idxs {
                        if let Some(deltas) = self.partial_matches(&input[*matched_idx..], *idx) {
                            new_matched_idxs.extend(deltas.iter().map(|didx| matched_idx + *didx));
                            lhs_found = true;
                        }
                    }
                    lhs_matched_idxs = new_matched_idxs;
                    if !lhs_found {
                        break;
                    }
                }

                let mut rhs_found = false;
                let mut rhs_matched_idxs = vec![0];
                for idx in rhs {
                    let mut new_matched_idxs = vec![];
                    for matched_idx in &rhs_matched_idxs {
                        if let Some(deltas) = self.partial_matches(&input[*matched_idx..], *idx) {
                            new_matched_idxs.extend(deltas.iter().map(|didx| matched_idx + *didx));
                            rhs_found = true;
                        }
                    }
                    rhs_matched_idxs = new_matched_idxs;
                    if !rhs_found {
                        break;
                    }
                }

                match (lhs_found, rhs_found) {
                    (false, false) => None,
                    (true, false) => Some(lhs_matched_idxs),
                    (false, true) => Some(rhs_matched_idxs),
                    (true, true) => Some(
                        lhs_matched_idxs
                            .iter()
                            .chain(rhs_matched_idxs.iter())
                            .copied()
                            .collect(),
                    ),
                }
            }
            Pattern::A(idxs) => {
                let mut found = false;
                let mut matched_idxs = vec![0];
                for idx in idxs {
                    let mut new_matched_idxs = vec![];
                    for matched_idx in &matched_idxs {
                        if let Some(deltas) = self.partial_matches(&input[*matched_idx..], *idx) {
                            new_matched_idxs.extend(deltas.iter().map(|didx| matched_idx + *didx));
                            found = true;
                        }
                    }
                    matched_idxs = new_matched_idxs;
                    if !found {
                        break;
                    }
                }

                if found {
                    return Some(matched_idxs);
                }

                None
            }
        }
    }
}

impl FromStr for Matcher {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();

        let mut patterns = HashMap::new();

        for line in lines {
            let line = line.trim();

            let mut idx_pattern_iter = line.split(": ");

            let idx = idx_pattern_iter
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            let pattern = idx_pattern_iter.next().unwrap().trim();

            let mut pattern_iter = pattern.split(" | ");
            match (pattern_iter.next(), pattern_iter.next()) {
                (Some(lhs), Some(rhs)) => {
                    let lhs_idxs: Vec<_> = lhs
                        .split_ascii_whitespace()
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect();
                    let rhs_idxs: Vec<_> = rhs
                        .split_ascii_whitespace()
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect();

                    patterns.insert(idx, Pattern::O(lhs_idxs, rhs_idxs));
                }
                (Some(pattern), None) => {
                    let mut cs = pattern.chars();
                    match [cs.next(), cs.next(), cs.next(), cs.next()] {
                        [Some('"'), Some(c), Some('"'), None] => {
                            patterns.insert(idx, Pattern::C(vec![c]));
                        }
                        _ => {
                            let idxs: Vec<_> = pattern
                                .split_ascii_whitespace()
                                .map(|c| c.parse::<usize>().unwrap())
                                .collect();
                            patterns.insert(idx, Pattern::A(idxs));
                        }
                    }
                }
                _ => return Err(()),
            }
        }

        Ok(Matcher { patterns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher() {
        let patterns = vec![
            Pattern::A(vec![4, 1, 5]),
            Pattern::O(vec![2, 3], vec![3, 2]),
            Pattern::O(vec![4, 4], vec![5, 5]),
            Pattern::O(vec![4, 5], vec![5, 4]),
            Pattern::C(vec!['a']),
            Pattern::C(vec!['b']),
        ];

        let patterns = patterns
            .iter()
            .enumerate()
            .map(|(idx, t)| (idx, t.clone()))
            .collect();

        let matcher = Matcher { patterns };

        assert_eq!(matcher.matches("ababbb"), true);
        assert_eq!(matcher.matches("abbbab"), true);

        assert_eq!(matcher.matches("bababa"), false);
        assert_eq!(matcher.matches("aaabbb"), false);
        assert_eq!(matcher.matches("aaaabbb"), false);
    }

    #[test]
    fn test_matcher_with_loop() {
        let patterns = vec![
            Pattern::O(vec![2], vec![1, 0]),
            Pattern::C(vec!['a']),
            Pattern::C(vec!['b']),
        ];

        let patterns = patterns
            .iter()
            .enumerate()
            .map(|(idx, t)| (idx, t.clone()))
            .collect();

        let matcher = Matcher { patterns };

        assert_eq!(matcher.matches("b"), true);
        assert_eq!(matcher.matches("ab"), true);
        assert_eq!(matcher.matches("aab"), true);
        assert_eq!(matcher.matches("aaab"), true);
    }

    #[test]
    fn test_a_loop_long() {
        let matcher = "42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: \"a\"
        11: 42 31 | 42 11 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: \"b\"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42 | 42 8
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1"
            .parse::<Matcher>()
            .unwrap();

        assert_eq!(
            matcher.matches("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
            true
        );
    }
}

use std::io::{self, Read};

use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): (.+)").unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let part_1 = &buffer
        .lines()
        .filter(|line| match re.captures(line) {
            Some(captures) => {
                let min_matching_chars: usize = (&captures[1]).parse().unwrap();
                let max_matching_chars: usize = (&captures[2]).parse().unwrap();
                let target_char = &captures[3].chars().next().unwrap();
                let password = &captures[4];

                let matching_chars = password.chars().filter(|char| char == target_char).count();

                min_matching_chars <= matching_chars && matching_chars <= max_matching_chars
            }
            None => false,
        })
        .count();

    let part_2 = &buffer
        .lines()
        .filter(|line| match re.captures(line) {
            Some(captures) => {
                let first_position: usize = (&captures[1]).parse().unwrap();
                let second_position: usize = (&captures[2]).parse().unwrap();
                let target_char = &captures[3].chars().next().unwrap();
                let password = &captures[4];

                let matching_chars = password
                    .chars()
                    .enumerate()
                    .filter(|(idx, char)| {
                        (idx + 1 == first_position || idx + 1 == second_position)
                            && char == target_char
                    })
                    .count();

                matching_chars == 1
            }
            None => false,
        })
        .count();

    println!("{}", part_1);
    println!("{}", part_2);

    Ok(())
}

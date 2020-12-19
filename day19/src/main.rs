use std::io::{self, Read};

use day19::Matcher;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut groups = buffer.split("\n\n");

    let rules = groups.next().unwrap();

    let matcher_part_1 = rules.parse::<Matcher>().unwrap();

    let rules_part_2 = rules
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let matcher_part_2 = rules_part_2.parse::<Matcher>().unwrap();

    let candidates = groups.next().unwrap();

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    for line in candidates.lines() {
        if matcher_part_1.matches(line) {
            sum_part_1 += 1;
        }
        if matcher_part_2.matches(line) {
            sum_part_2 += 1;
        }
    }

    println!("{}", sum_part_1);
    println!("{}", sum_part_2);

    Ok(())
}

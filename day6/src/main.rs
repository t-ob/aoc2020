use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    for group in buffer.split("\n\n") {
        // Part 1
        part_1_sum += group
            .replace("\n", "")
            .chars()
            .collect::<HashSet<_>>()
            .len();

        // Part 2
        let mut group_answer_sets = group.lines().map(|s| s.chars().collect::<HashSet<_>>());
        if let Some(first_answer_set) = group_answer_sets.next() {
            part_2_sum += group_answer_sets
                .fold(first_answer_set, |accumulated_intersection, answer_set| {
                    accumulated_intersection
                        .intersection(&answer_set)
                        .cloned()
                        .collect()
                })
                .len()
        }
    }

    println!("{}", part_1_sum);
    println!("{}", part_2_sum);

    Ok(())
}

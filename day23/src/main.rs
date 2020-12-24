use std::io::{self, Read};

fn play(values: &Vec<usize>, rounds: usize) -> Vec<usize> {
    let max_val = values.iter().max().unwrap();

    let mut linked_cups = values
        .iter()
        .enumerate()
        .map(|(idx, x)| (*x, (idx + 1) % max_val))
        .collect::<Vec<_>>();

    let mut lookup = vec![0; *max_val + 1];
    for (idx, val) in values.iter().enumerate() {
        lookup[*val] = idx;
    }

    let mut turn = 0;
    let mut idx = 0;
    while turn < rounds {
        let cup = linked_cups[idx];

        let next_cup_1 = linked_cups[cup.1];
        let next_cup_2 = linked_cups[next_cup_1.1];
        let next_cup_3 = linked_cups[next_cup_2.1];

        let cup_val = cup.0;

        let mut dest_val = cup_val - 1;
        if dest_val == 0 {
            dest_val = *max_val;
        }

        while dest_val == next_cup_1.0 || dest_val == next_cup_2.0 || dest_val == next_cup_3.0 {
            dest_val -= 1;
            if dest_val == 0 {
                dest_val = *max_val;
            }
        }

        let dest_cup = linked_cups[lookup[dest_val]];

        let next_val = next_cup_3.1;

        linked_cups[lookup[next_cup_3.0]].1 = dest_cup.1;
        linked_cups[lookup[dest_cup.0]].1 = cup.1;
        linked_cups[lookup[cup.0]].1 = next_val;

        idx = next_val;
        turn += 1;
    }

    let mut result = Vec::new();

    let mut idx = 0;
    while result.len() < values.len() {
        let cup = linked_cups[idx];
        result.push(cup.0);
        idx = cup.1;
    }

    result
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Part 1
    let vals_part_1 = buffer
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<_>().unwrap())
        .collect::<Vec<_>>();

    let part_1 = play(&vals_part_1, 100);
    println!(
        "{}",
        part_1
            .iter()
            .cycle()
            .skip_while(|x| **x != 1)
            .skip(1)
            .take(vals_part_1.len() - 1)
            .map(|x| x.to_string())
            .collect::<String>()
    );

    // Part 2
    let mut vals_part_2 = vals_part_1.clone();
    for x in 10..=1000000 {
        vals_part_2.push(x);
    }

    let part_2 = play(&vals_part_2, 10000000);
    println!(
        "{}",
        part_2
            .iter()
            .cycle()
            .skip_while(|x| **x != 1)
            .skip(1)
            .take(2)
            .fold(1, |a, x| a * x)
    );

    Ok(())
}

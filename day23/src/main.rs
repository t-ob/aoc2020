use std::io::{self, Read};

fn play(values: &[usize], rounds: usize) -> Vec<usize> {
    let max_val = values.iter().max().unwrap();

    let mut next_cups = vec![0; *max_val + 1];
    for (val, next_val) in values.iter().cycle().zip(values.iter().cycle().skip(1)).take(values.len()) {
        next_cups[*val] = *next_val
    }

    let mut round = 0;
    let mut cup = values[0];
    while round < rounds {
        let next_cup_1 = next_cups[cup];
        let next_cup_2 = next_cups[next_cup_1];
        let next_cup_3 = next_cups[next_cup_2];

        let mut dest_cup = cup - 1;
        if dest_cup == 0 {
            dest_cup = *max_val;
        }

        while dest_cup == next_cup_1 || dest_cup == next_cup_2 || dest_cup == next_cup_3 {
            dest_cup -= 1;
            if dest_cup == 0 {
                dest_cup = *max_val;
            }
        }

        let next_round_cup = next_cups[next_cup_3];

        next_cups[next_cup_3] = next_cups[dest_cup];
        next_cups[dest_cup] = next_cups[cup];
        next_cups[cup] = next_round_cup;

        cup = next_round_cup;
        round += 1;
    }

    let mut result = Vec::new();

    let mut cup = values[0];
    while result.len() < values.len() {
        result.push(cup);
        cup = next_cups[cup];
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
    let mut vals_part_2 = vals_part_1;
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
            .product::<usize>()
    );

    Ok(())
}

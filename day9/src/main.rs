use std::{
    cmp::{max, min},
    env,
    io::{self, Read},
};

use day9::Preamble;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let preamble_size: usize = args[1].parse().unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<i64> = buffer.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    let (seed, rest) = input.split_at(preamble_size);

    let mut preamble = Preamble::new(seed);

    let mut first_invalid: Option<i64> = None;

    // Part 1
    for x in rest {
        if !preamble.is_valid(*x) {
            println!("{}", *x);
            first_invalid = Some(*x);
            break;
        }
        preamble.push(*x);
    }

    // Part 2
    if let Some(first_invalid) = first_invalid {
        let mut i = 0;
        let mut j = 1;

        let mut s = input[0] + input[1];
        while s != first_invalid {
            if s < first_invalid {
                j += 1;
                s += input[j];
            } else {
                s -= input[i];
                i += 1;
            }
        }

        let mut x = input[i];
        let mut y = input[i];

        for z in input[i + 1..=j].iter() {
            x = min(x, *z);
            y = max(y, *z);
        }

        println!("{}", x + y);
    } else {
        panic!("No invalid number found")
    }

    Ok(())
}

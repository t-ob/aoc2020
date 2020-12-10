use std::{
    collections::{BinaryHeap, HashMap},
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut jolts: BinaryHeap<_> = buffer.lines().map(|s| s.parse().unwrap()).collect();

    if jolts.is_empty() {
        panic!("Empty input")
    }

    jolts.push(0);
    let built_in_max = jolts.peek().unwrap() + 3;
    jolts.push(built_in_max);

    let mut ones = 0;
    let mut threes = 0;

    let mut num_ways: HashMap<i32, usize> = HashMap::new();
    num_ways.insert(built_in_max, 1);

    let mut higher_jolt = jolts.pop().unwrap();
    while !jolts.is_empty() {
        // Part 1
        let lower_jolt = jolts.pop().unwrap();

        if higher_jolt - lower_jolt == 1 {
            ones += 1;
        }
        if higher_jolt - lower_jolt == 3 {
            threes += 1;
        }

        // Part 2
        let num_ways_from = [lower_jolt + 1, lower_jolt + 2, lower_jolt + 3]
            .iter()
            .map(|jolt| num_ways.get(jolt).unwrap_or(&0))
            .fold(0, |x, y| x + y);
        num_ways.insert(lower_jolt, num_ways_from);

        higher_jolt = lower_jolt;
    }

    println!("{}", ones * threes);
    println!("{}", num_ways.get(&0).unwrap());

    Ok(())
}

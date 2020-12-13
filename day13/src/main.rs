use std::io::{self, Read};

use day13::chinese_remainder;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();

    if let Some(earliest_time) = lines.next() {
        if let Some(timetable) = lines.next() {
            let earliest_time = earliest_time.parse::<i128>().unwrap();
            let ids = timetable
                .split(',')
                .map(|s| s.parse::<_>().ok())
                .collect::<Vec<_>>();

            // Part 1
            let (first_bus_time, first_bus_id) = ids
                .iter()
                .filter(|id| **id != None)
                .map(|id| {
                    let id = id.unwrap();
                    if earliest_time % id == 0 {
                        return (earliest_time, id);
                    }
                    (earliest_time + id - (earliest_time % id), id)
                })
                .min()
                .unwrap();

            println!("{}", first_bus_id * (first_bus_time - earliest_time));

            // Part 2
            let linear_congruences = ids
                .iter()
                .enumerate()
                .filter(|(_, id)| **id != None)
                .map(|(t, n)| (-(t as i128), n.unwrap()))
                .collect::<Vec<_>>();

            if let Some(solution) = chinese_remainder(&linear_congruences) {
                println!("{:?}", solution);
            }
        } else {
            panic!("Empty input")
        }
    } else {
        panic!("Empty input")
    }

    Ok(())
}

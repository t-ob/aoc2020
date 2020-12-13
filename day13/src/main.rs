use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();

    if let Some(earliest_time) = lines.next() {
        if let Some(timetable) = lines.next() {
            let earliest_time = earliest_time.parse::<i64>().unwrap();
            let ids = timetable.split(',').map(|s| s.parse::<i64>().ok());

            let xxx = ids
                .filter(|id| *id != None)
                .map(|id| {
                    let id = id.unwrap();
                    if earliest_time % id == 0 {
                        return (earliest_time, id);
                    }
                    (earliest_time + id - (earliest_time % id), id)
                })
                .min()
                .unwrap();

            println!("{:?}", xxx.1 * (xxx.0 - earliest_time));
        } else {
            panic!("Empty input")
        }
    } else {
        panic!("Empty input")
    }

    Ok(())
}

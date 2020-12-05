use std::{
    cmp::max,
    cmp::min,
    io::{self, Read},
};

use day5::compute_seat_id;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut curr_min_seat_id = None;
    let mut curr_max_seat_id = None;
    let mut sum = 0;
    for boarding_pass in buffer.lines() {
        if let Ok(seat_id) = compute_seat_id(boarding_pass) {
            sum += seat_id;
            curr_min_seat_id = Some(min(curr_min_seat_id.unwrap_or(seat_id), seat_id));
            curr_max_seat_id = Some(max(curr_max_seat_id.unwrap_or(seat_id), seat_id));
        }
    }

    println!("{}", curr_max_seat_id.unwrap_or(0));

    if let (Some(min_seat_id), Some(max_seat_id)) = (curr_min_seat_id, curr_max_seat_id) {
        let delta = max_seat_id - min_seat_id;

        // a + (a + 1) + ... + (a + n) = (n + 1) * a + (n * (n + 1)) / 2
        let arithmetic_sum = (delta + 1) * min_seat_id + ((delta * (delta + 1)) / 2);

        let missing_seat_id = arithmetic_sum - sum;
        println!("{}", missing_seat_id);
    }

    Ok(())
}

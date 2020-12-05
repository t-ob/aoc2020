pub fn compute_seat_id(boarding_pass: &str) -> Result<u32, String> {
    let chars = boarding_pass.chars();

    let mut count = 0;
    let mut seat_id = 0;
    for (foo, char) in chars.enumerate() {
        count += 1;
        seat_id <<= 1;
        match (foo < 7, char) {
            (true, 'B') => seat_id |= 1,
            (true, 'F') => {}
            (false, 'R') => seat_id |= 1,
            (false, 'L') => {}
            _ => return Err(format!("Invalid character: {}", char)),
        }
    }

    if count != 10 {
        return Err(format!(
            "Incorrect number of characters. Got {}, expected 10",
            count
        ));
    }

    Ok(seat_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(compute_seat_id(&"BFFFBBFRRR".to_string()), Ok(567));
        assert_eq!(compute_seat_id(&"FFFBBBFRRR".to_string()), Ok(119));
        assert_eq!(compute_seat_id(&"BBFFBBFRLL".to_string()), Ok(820));
    }

    #[test]
    fn test_seat_id_bad_input() {
        assert_eq!(
            compute_seat_id(&"BFFFBBFRRRR".to_string()),
            Err("Incorrect number of characters. Got 11, expected 10".to_string())
        );
        assert_eq!(
            compute_seat_id(&"BFFFBBFRR".to_string()),
            Err("Incorrect number of characters. Got 9, expected 10".to_string())
        );
        assert_eq!(
            compute_seat_id(&"BFFFBBFRRZ".to_string()),
            Err("Invalid character: Z".to_string())
        );
    }
}

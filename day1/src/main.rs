use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

fn main() {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("input");
    let file = File::open(&path_buf).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut two_sum_seen: HashSet<i32> = HashSet::new();

    for line in lines {
        let y: i32 = line.unwrap().parse().unwrap();

        // Check if y is in set
        // If so, that means we have seen a previous x such that
        // 2020 - x = y
        // So x = 2020 - y, so print y * (2020 - y) and quit

        if two_sum_seen.contains(&y) {
            println!("{}", y * (2020 - y));
            break;
        }

        // If not, add 2020 - y to set

        two_sum_seen.insert(2020 - y);
    }

    let file = File::open(&path_buf).unwrap();
    let lines = io::BufReader::new(file).lines();

    // For the three-sum version, we use a (sorted) vector of values
    let mut three_sum_vals: Vec<i32> = lines.map(|line| line.unwrap().parse().unwrap()).collect();
    three_sum_vals.sort();

    // Start from the leastmost element, and then use a sliding window from either end of the remainder to the right
    let mut found = false;
    for (idx, a) in three_sum_vals.iter().enumerate() {
        if found {
            break;
        }
        if idx >= three_sum_vals.len() - 2 {
            break;
        }
        let mut i = idx + 1;
        let mut j = three_sum_vals.len() - 1;
        while i < j {
            let b = three_sum_vals[i];
            let c = three_sum_vals[j];
            if a + b + c == 2020 {
                println!("{}", a * b * c);
                found = true;
                break;
            } else if a + b + c > 2020 {
                // Overshot: c must be too big
                j = j - 1;
            } else {
                // Undershot: b must be too small
                i = i + 1;
            }
        }
    }
}

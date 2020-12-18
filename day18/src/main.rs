use std::io::{self, Read};

use day18::{eval, read, Expr, Mode};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let part_1: i64 = buffer
        .lines()
        .map(|s| eval(Expr::new(&read(s.trim(), Mode::InOrder)).unwrap()))
        .sum();
    println!("{}", part_1);

    let part_2: i64 = buffer
        .lines()
        .map(|s| eval(Expr::new(&read(s.trim(), Mode::AdditionTakesPrecedence)).unwrap()))
        .sum();
    println!("{}", part_2);

    Ok(())
}

use std::io::{self, Read};

static P: usize = 20201227;

fn main() -> io::Result<()> {
    let mut exponents = vec![0; P];
    let mut discrete_log = vec![0; P];

    let mut n = 1;
    for e in 1..P {
        n = (n * 7) % P;
        exponents[e] = n;
        discrete_log[n] = e;
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();

    let card_public_key = lines.next().unwrap().parse::<usize>().unwrap();
    let door_public_key = lines.next().unwrap().parse::<usize>().unwrap();

    let e_card = discrete_log[card_public_key];
    let e_door = discrete_log[door_public_key];

    let pk = exponents[(e_card * e_door) % (P - 1)];
    println!("{}", pk);

    Ok(())
}

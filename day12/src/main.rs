use std::io::{self, Read};

use day12::{Action, Vector};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let north = Vector(0, 1);
    let east = Vector(1, 0);
    let south = Vector(0, -1);
    let west = Vector(-1, 0);

    // Part 1
    let mut ship_part_1 = Vector(0, 0);
    let mut ship_part_1_direction = Vector(1, 0);

    // Part 2
    let mut ship_part_2 = Vector(0, 0);
    let mut ship_part_2_waypoint = Vector(10, 1);

    for line in buffer.lines() {
        let action = line.parse::<Action>().unwrap();
        match action {
            Action::Forward(n) => {
                ship_part_1 += n * ship_part_1_direction;
                ship_part_2 += n * ship_part_2_waypoint;
            }
            Action::North(n) => {
                ship_part_1 += n * north;
                ship_part_2_waypoint += n * north;
            }
            Action::East(n) => {
                ship_part_1 += n * east;
                ship_part_2_waypoint += n * east;
            }
            Action::South(n) => {
                ship_part_1 += n * south;
                ship_part_2_waypoint += n * south;
            }
            Action::West(n) => {
                ship_part_1 += n * west;
                ship_part_2_waypoint += n * west;
            }
            Action::Left(r) => {
                ship_part_1_direction.rotate(r);
                ship_part_2_waypoint.rotate(r);
            }
            Action::Right(r) => {
                ship_part_1_direction.rotate(r.inverse());
                ship_part_2_waypoint.rotate(r.inverse());
            }
        }
    }

    println!("{}", ship_part_1.l1_norm());
    println!("{}", ship_part_2.l1_norm());

    Ok(())
}

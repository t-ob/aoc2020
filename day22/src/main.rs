use std::io::{self, Read};

use day22::{Game, GameMode, GameState};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut groups = buffer.split("\n\n");

    if let (Some(player_1_input), Some(player_2_input)) = (groups.next(), groups.next()) {
        let player_1_deck = player_1_input
            .lines()
            .skip(1)
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let player_2_deck = player_2_input
            .lines()
            .skip(1)
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut game_part_1 = Game::new(GameMode::Basic, &player_1_deck, &player_2_deck);
        game_part_1.play();
        match game_part_1.state() {
            GameState::Complete(_, score) => println!("{}", score),
            _ => panic!("Shouldn't happen"),
        }

        let mut game_part_2 = Game::new(GameMode::Recursive, &player_1_deck, &player_2_deck);
        game_part_2.play();
        match game_part_2.state() {
            GameState::Complete(_, score) => println!("{}", score),
            _ => panic!("Shouldn't happen"),
        }
    }

    Ok(())
}

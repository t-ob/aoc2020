use std::collections::{HashSet, VecDeque};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Basic,
    Recursive,
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    InProgress,
    Complete(Player, u32),
}

pub struct Game {
    mode: GameMode,
    state: GameState,
    p1_hand: VecDeque<u32>,
    p2_hand: VecDeque<u32>,
    seen_signatures: HashSet<(u32, u32)>,
}

impl Game {
    pub fn new(mode: GameMode, p1_hand: &[u32], p2_hand: &[u32]) -> Game {
        let state = GameState::InProgress;
        let p1_hand = p1_hand.iter().map(|x| *x).collect();
        let p2_hand = p2_hand.iter().map(|x| *x).collect();
        let seen_signatures = HashSet::new();
        Game {
            mode,
            state,
            p1_hand,
            p2_hand,
            seen_signatures,
        }
    }
    pub fn play(&mut self) {
        loop {
            match self.state {
                GameState::Complete(_, _) => break,
                GameState::InProgress => {
                    let signature = self.signature();
                    if self.seen_signatures.contains(&signature) {
                        self.state = GameState::Complete(Player::P1, self.score_hand(Player::P1));
                        continue;
                    }
                    self.seen_signatures.insert(signature);

                    if self.hand_size(Player::P1) == 0 {
                        self.state = GameState::Complete(Player::P2, self.score_hand(Player::P2));
                        continue;
                    }
                    if self.hand_size(Player::P2) == 0 {
                        self.state = GameState::Complete(Player::P1, self.score_hand(Player::P1));
                        continue;
                    }

                    let (u, v) = (
                        self.get_top_card(Player::P1).unwrap(),
                        self.get_top_card(Player::P2).unwrap(),
                    );

                    let p1_hand_size = self.hand_size(Player::P1);
                    let p2_hand_size = self.hand_size(Player::P2);
                    match (
                        self.mode,
                        u as usize <= p1_hand_size && v as usize <= p2_hand_size,
                    ) {
                        (GameMode::Basic, _) | (GameMode::Recursive, false) => match u.cmp(&v) {
                            std::cmp::Ordering::Less => {
                                self.push_card(Player::P2, v);
                                self.push_card(Player::P2, u);
                            }
                            std::cmp::Ordering::Equal => {
                                self.push_card(Player::P1, u);
                                self.push_card(Player::P2, v);
                            }
                            std::cmp::Ordering::Greater => {
                                self.push_card(Player::P1, u);
                                self.push_card(Player::P1, v);
                            }
                        },
                        (GameMode::Recursive, true) => {
                            let mut sub_game = Box::new(Game {
                                mode: self.mode,
                                state: GameState::InProgress,
                                p1_hand: self.copy_hand(Player::P1, u as usize),
                                p2_hand: self.copy_hand(Player::P2, v as usize),
                                seen_signatures: HashSet::new(),
                            });
                            sub_game.play();
                            match sub_game.state() {
                                GameState::InProgress => panic!("Shouldn't match this"),
                                GameState::Complete(Player::P1, _) => {
                                    self.push_card(Player::P1, u);
                                    self.push_card(Player::P1, v);
                                }
                                GameState::Complete(Player::P2, _) => {
                                    self.push_card(Player::P2, v);
                                    self.push_card(Player::P2, u);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    fn hand_size(&self, player: Player) -> usize {
        match player {
            Player::P1 => self.p1_hand.len(),
            Player::P2 => self.p2_hand.len(),
        }
    }

    fn get_top_card(&mut self, player: Player) -> Option<u32> {
        match player {
            Player::P1 => self.p1_hand.pop_front(),
            Player::P2 => self.p2_hand.pop_front(),
        }
    }

    fn push_card(&mut self, player: Player, card: u32) {
        match player {
            Player::P1 => self.p1_hand.push_back(card),
            Player::P2 => self.p2_hand.push_back(card),
        }
    }

    fn copy_hand(&self, player: Player, hand_size: usize) -> VecDeque<u32> {
        match player {
            Player::P1 => self.p1_hand.clone().into_iter().take(hand_size).collect(),
            Player::P2 => self.p2_hand.clone().into_iter().take(hand_size).collect(),
        }
    }

    fn score_hand(&self, player: Player) -> u32 {
        match player {
            Player::P1 => self.p1_hand.iter().rev().zip(1..).map(|(x, y)| x * y).sum(),
            Player::P2 => self.p2_hand.iter().rev().zip(1..).map(|(x, y)| x * y).sum(),
        }
    }

    fn signature(&self) -> (u32, u32) {
        let mut p1_hand_hash = Sha256::new();
        for card in self.p1_hand.iter() {
            p1_hand_hash.update(card.to_ne_bytes());
        }

        let mut p2_hand_hash = Sha256::new();
        for card in self.p2_hand.iter() {
            p2_hand_hash.update(card.to_ne_bytes());
        }

        let mut p1_hash_high_bytes: [u8; 4] = [0; 4];
        p1_hash_high_bytes.copy_from_slice(&p1_hand_hash.finalize()[..4]);
        let p1_component: u32 = (p1_hash_high_bytes[0] as u32) << 24
            | (p1_hash_high_bytes[1] as u32) << 16
            | (p1_hash_high_bytes[2] as u32) << 8
            | (p1_hash_high_bytes[3] as u32);

        let mut pw_hash_high_bytes: [u8; 4] = [0; 4];
        pw_hash_high_bytes.copy_from_slice(&p2_hand_hash.finalize()[..4]);
        let p2_component: u32 = (pw_hash_high_bytes[0] as u32) << 24
            | (pw_hash_high_bytes[1] as u32) << 16
            | (pw_hash_high_bytes[2] as u32) << 8
            | (pw_hash_high_bytes[3] as u32);

        (p1_component, p2_component)
    }
}

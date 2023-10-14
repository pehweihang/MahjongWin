use std::collections::HashMap;

use super::tile::{Meld, Tile};

#[derive(Debug)]
pub struct Player {
    hand: HashMap<Tile, u8>,
    flowers: Vec<Tile>,
    animals: Vec<Tile>,
    melds: Vec<Meld>,
}

impl Player {
    fn draw(&mut self, tile: Tile) {
        match self.hand.get(&tile) {
            Some(count) => {
                self.hand.insert(tile, count + 1);
            }
            None => {
                self.hand.insert(tile, 1);
            }
        }
    }

    // Chi can be performed if an opponent's tile can make a set of 3 consecutive tiles. Example: 3, 4, 5
    pub fn canChi(&self, tile: Tile) -> Option<Vec<Action>> {
        match tile {
            Tile::Wan(v) | Tile::Suo(v) | Tile::Tong(v) => {
                let mut actions = vec![];
                for (a, b) in vec![(-2, -1), (-1, 1), (1, 2)] {
                    if v + a > 0 && v + b <= 9 {
                        todo!()
                    }
                }
                if actions.is_empty() {
                    None
                } else {
                    Some(actions)
                }
            }
            _ => None,
        }
    }

    // Pong can be performed if an opponent's tile can make a set of 3 duplicate tiles. Example: 7, 7, 7
    fn canPong(&self, tile: Tile) -> Option<Vec<Action>> {
        todo!()
    }

    // Gang can be performed if an opponent's tile can make a set of 4 duplicate tiles. Example: 7, 7, 7, 7
    fn canGang(&self, tile: Tile) -> Option<Action> {
        todo!()
    }

    // AnGang can be performed if there are 4 duplicate tiles in player's hand. Example: 2, 2, 2, 2
    fn canAnGang(&self) -> Option<Action> {
        todo!()
    }
}

enum Action {}

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
    fn draw(&mut self, t: Tile) {
        match self.hand.get(&t) {
            Some(count) => {
                self.hand.insert(t, count + 1);
            }
            None => {
                self.hand.insert(t, 1);
            }
        }
    }

    pub fn canChi(&self, t: Tile) -> Option<Vec<Action>> {
        match Tile {
            Tile::Wan(v) | Tile::Suo(v) | Tile::Tong(v) => {
                let mut actions = vec![];
                for (a, b) in vec![(-2, -1), (-1, 1), (1, 2)] {
                    if v + a > 0 && v + b <= 9 {

                    }
                };
                if actions.is_empty() {
                    None
                }
                else {
                    Some(actions)
                }
            }
            _ => None,
        }
    }

    fn canPong(&self, t: Tile) -> Option<Vec<Action>> {
        todo!()
    }

    fn canGang(&self, t: Tile) -> Option<Action> {
        todo!()
    }

    fn canAnGang(&self) -> Option<Action> {
        todo!()
    }
}

enum Action {}

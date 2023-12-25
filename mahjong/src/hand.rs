use std::collections::{HashMap, HashSet};

use crate::{
    error::MahjongError,
    meld::{Meld, MeldType},
    tile::Tile,
};

#[derive(Debug, Default)]
pub struct Hand {
    hand: HashMap<Tile, u8>,
    _melds: Vec<Meld>,
    _bonus_tiles: HashSet<Tile>,
    seen_tiles: HashSet<Tile>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn draw(&mut self, tile: &Tile) {
        self.hand
            .entry(tile.to_owned())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        self.seen_tiles.clear();
    }

    pub fn discard(&mut self, tile: &Tile) -> Result<(), MahjongError> {
        if let Some(v) = self.hand.get(tile) {
            if *v == 1 {
                self.hand.remove(tile);
            } else {
                self.hand.insert(tile.to_owned(), *v - 1);
            }
        } else {
            return Err(MahjongError::TileNotFoundError(tile.to_owned()));
        }
        self.seen_tiles.insert(tile.to_owned());
        Ok(())
    }

    pub fn get_melds(&self, tile: &Tile) -> Result<Vec<Meld>, MahjongError> {
        if !tile.is_playable() {
            return Err(MahjongError::TileNotPlayableError(tile.suit()));
        }
        let mut poss_melds = vec![];
        if let Some(num) = self.hand.get(tile) {
            if *num >= 2 {
                poss_melds.push(Meld::new(
                    vec![tile.to_owned(), tile.to_owned(), tile.to_owned()],
                    MeldType::Pong,
                )?);
            }
            if *num >= 3 {
                poss_melds.push(Meld::new(
                    vec![
                        tile.to_owned(),
                        tile.to_owned(),
                        tile.to_owned(),
                        tile.to_owned(),
                    ],
                    MeldType::Gang,
                )?);
            }
        }
        let prev = tile.prev();
        let prev_prev = prev.and_then(|t| t.prev());
        let next = tile.next();
        let next_next = next.and_then(|t| t.next());

        let tiles_to_check = [prev_prev, prev, next, next_next];
        let mut it = tiles_to_check.windows(2);
        while let Some([t1, t2]) = it.next() {
            if let (Some(t1), Some(t2)) = (t1, t2) {
                let mut tiles = vec![t1.to_owned(), t2.to_owned(), tile.to_owned()];
                tiles.sort();
                if self.hand.contains_key(t1) && self.hand.contains_key(t2) {
                    poss_melds.push(Meld::new(tiles, MeldType::Chi)?);
                }
            }
        }
        Ok(poss_melds)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use claim::assert_err;

    use crate::{
        meld::{Meld, MeldType},
        tile::{Tile, TileValue},
    };

    use super::Hand;

    #[test]
    fn test_draw_tile_ok() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::One));
        assert_eq!(hand.hand, HashMap::from([(Tile::Wan(TileValue::One), 1)]));
        hand.draw(&Tile::Wan(TileValue::One));
        assert_eq!(hand.hand, HashMap::from([(Tile::Wan(TileValue::One), 2)]));
    }

    #[test]
    fn test_no_seen_tiles_after_draw() {
        let mut hand = Hand::new();
        hand.seen_tiles.insert(Tile::Wan(TileValue::One));
        hand.draw(&Tile::Suo(TileValue::One));
        assert_eq!(hand.seen_tiles.len(), 0);
    }

    #[test]
    fn test_discard_tile_ok() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::One));
        hand.draw(&Tile::Wan(TileValue::One));
        hand.discard(&Tile::Wan(TileValue::One)).unwrap();
        assert_eq!(hand.hand, HashMap::from([(Tile::Wan(TileValue::One), 1)]));
        hand.discard(&Tile::Wan(TileValue::One)).unwrap();
        assert_eq!(hand.hand, HashMap::from([]));
    }

    #[test]
    fn test_discard_tile_not_in_hand_throws_error() {
        let mut hand = Hand::new();
        assert_err!(hand.discard(&Tile::Wan(TileValue::One)));
    }

    #[test]
    fn test_seen_tile_after_discard() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::One));
        hand.discard(&Tile::Wan(TileValue::One)).unwrap();
        assert!(hand.seen_tiles.contains(&Tile::Wan(TileValue::One)));
    }

    #[test]
    fn test_get_melds() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Five));
        hand.draw(&Tile::Wan(TileValue::Six));

        let mut melds = hand.get_melds(&Tile::Wan(TileValue::Four)).unwrap();
        melds.sort();

        let mut correct_melds = vec![
            Meld::new(
                vec![
                    Tile::Wan(TileValue::Two),
                    Tile::Wan(TileValue::Three),
                    Tile::Wan(TileValue::Four),
                ],
                MeldType::Chi,
            ).unwrap(),
            Meld::new(
                vec![
                    Tile::Wan(TileValue::Three),
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Five),
                ],
                MeldType::Chi,
            ).unwrap(),
            Meld::new(
                vec![
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Five),
                    Tile::Wan(TileValue::Six),
                ],
                MeldType::Chi,
            ).unwrap(),
            Meld::new(
                vec![
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                ],
                MeldType::Pong,
            ).unwrap(),
            Meld::new(
                vec![
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                ],
                MeldType::Gang,
            ).unwrap(),
        ];
        correct_melds.sort();
        assert_eq!(melds, correct_melds);
    }
}

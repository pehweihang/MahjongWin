use std::collections::{HashMap, HashSet};

use crate::{error::MahjongError, meld::Meld, tile::Tile};
use anyhow::{Context, Result};

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

    pub fn discard(&mut self, tile: &Tile) -> Result<()> {
        if let Some(v) = self.hand.get(tile) {
            if *v == 1 {
                self.hand.remove(tile);
            } else {
                self.hand.insert(tile.to_owned(), *v - 1);
            }
        } else {
            return Err(MahjongError::TileNotFoundError(tile.to_owned()))
                .context("Cannot discard tile not in hand");
        }
        self.seen_tiles.insert(tile.to_owned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use claim::assert_err;

    use crate::tile::{Tile, TileValue};

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
}

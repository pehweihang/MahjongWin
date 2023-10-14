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
    /// Draw and place a tile into the player's hand
    ///
    /// # Arguments
    ///
    /// * `tile` - drawn tile
    pub fn draw(&mut self, tile: Tile) {
        match self.hand.get(&tile) {
            Some(count) => {
                self.hand.insert(tile, count + 1);
            }
            None => {
                self.hand.insert(tile, 1);
            }
        }
    }

    /// A player can perfrom the action 'chi' if they can form a sequence of 3 consecutive tiles
    /// from a discarded tile of another player
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile the check if 'chi' can be performed on
    pub fn can_chi(&self, tile: Tile) -> Vec<Meld> {
        let mut possible_melds = vec![];
        if let Tile::Wan(_) | Tile::Suo(_) | Tile::Tong(_) = tile {
            let tile_as_int: i8 = tile.into();
            for (a, b) in &[(-2, -1), (-1, 1), (1, 2)] {
                let tile_a = (tile_as_int + a).try_into().unwrap();
                let tile_b = (tile_as_int + b).try_into().unwrap();
                if self.hand.contains_key(&tile_a) && self.hand.contains_key(&tile_b) {
                    possible_melds.push(Meld::Chi(tile_a, tile_b, tile_as_int.try_into().unwrap()))
                }
            }
        };
        possible_melds
    }
}

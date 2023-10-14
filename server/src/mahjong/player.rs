use std::collections::HashMap;

use super::tile::{Meld, Tile};

#[derive(Debug, Default)]
pub struct Player {
    hand: HashMap<Tile, u8>,
    flowers: Vec<Tile>,
    animals: Vec<Tile>,
    melds: Vec<Meld>,

    chips: i32,
}

impl Player {
    pub fn new(chips: i32) -> Self {
        Self {
            chips,
            ..Default::default()
        }
    }
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

    pub fn can_pong(&self, tile: Tile) -> Vec<Meld> {
        todo!()
    }

    pub fn can_gang(&self, tile: Tile) -> Vec<Meld> {
        todo!()
    }

    pub fn can_angang(&self) -> Vec<Meld> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mahjong::tile::{Meld, Tile};

    use super::Player;

    #[test]
    fn test_can_chi() {
        let hand_before_chi = HashMap::from([
            (Tile::Wan(1), 1),
            (Tile::Wan(2), 1),
            (Tile::Wan(8), 1),
            (Tile::Wan(9), 1),
            (Tile::Suo(1), 1),
            (Tile::Suo(3), 1),
            (Tile::Suo(6), 1),
            (Tile::Suo(7), 1),
            (Tile::Suo(8), 1),
            (Tile::Suo(9), 1),
        ]);
        let player = Player {
            hand: hand_before_chi,
            ..Default::default()
        };

        // (tile to check, possible chi's)
        let test_cases = vec![
            (
                // chi right
                Tile::Wan(3),
                vec![Meld::Chi(Tile::Wan(1), Tile::Wan(2), Tile::Wan(3))],
            ),
            // chi left
            (
                Tile::Wan(7),
                vec![Meld::Chi(Tile::Wan(8), Tile::Wan(9), Tile::Wan(7))],
            ),
            // chi middle
            (
                Tile::Suo(2),
                vec![Meld::Chi(Tile::Suo(1), Tile::Suo(3), Tile::Suo(2))],
            ),
            // chi multiple
            (
                Tile::Suo(7),
                vec![
                    Meld::Chi(Tile::Suo(6), Tile::Suo(8), Tile::Suo(7)),
                    Meld::Chi(Tile::Suo(8), Tile::Suo(9), Tile::Suo(7)),
                ],
            ),
            // cannot chi
            (Tile::Tong(1), vec![]),
        ];

        for (tile, melds) in test_cases {
            assert_eq!(player.can_chi(tile), melds);
        }
    }
}

use std::collections::{HashMap, HashSet};

use super::tile::{Meld, Tile};

#[derive(Debug, Default)]
pub struct Player {
    hand: HashMap<Tile, u8>,
    flowers: Vec<Tile>,
    animals: Vec<Tile>,
    melds: Vec<Meld>,

    skipped_tiles: HashSet<Tile>,

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
        self.skipped_tiles.clear();
    }

    /// Keep track of tiles discarded by other players. The player cannot 'pong' or 'hu' from a
    /// tile that is skipped by the player until the player draws again.
    ///
    /// # Arguments
    ///
    /// * `tile` - tile skipped by the player
    pub fn skipped_tile(&mut self, tile: Tile) {
        self.skipped_tiles.insert(tile);
    }

    /// A player can perform the action 'chi' if they can form a sequence of 3 consecutive tiles
    /// from a discarded tile of another player
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'chi' can be performed on
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

    /// A player can perform the action 'pong' if they can form a sequence of 3 identical tiles
    /// from a discarded tile of another player
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'pong' can be performed on
    pub fn can_pong(&self, tile: Tile) -> Vec<Meld> {
        let mut possible_melds = vec![];

        if self.skipped_tiles.contains(&tile) {
            return possible_melds
        }

        if let Some(&value) = self.hand.get(&tile) {
            if value >= 2 {
                possible_melds.push(Meld::Pong(tile))
            }
        }

        possible_melds
    }

    /// A player can perform the action 'gang' if they can form a sequence of 4 identical tiles
    /// from a discarded tile of another player
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'gang' can be performed on
    pub fn can_gang(&self, tile: Tile) -> Vec<Meld> {
        let mut possible_melds = vec![];

        if let Some(&value) = self.hand.get(&tile) {
            if value == 3 {
                possible_melds.push(Meld::Gang(tile))
            }
        }

        possible_melds
    }

    /// A player can perform the action 'angang' if they can form a sequence of 4 identical tiles from their hand
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'gang' can be performed on
    pub fn can_angang(&self) -> Vec<Meld> {
        let mut possible_melds = vec![];

        for (key, &value) in &self.hand {
            if value == 4 {
                possible_melds.push(Meld::AnGang(key.clone()))
            }
        }

        return possible_melds;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mahjong::tile::{DragonType, HuaType, Meld, Tile};

    use super::Player;

    #[test]
    fn test_skipped_tile() {
        let hand = HashMap::from([(Tile::Wan(2), 2)]);
        let mut player = Player {hand, ..Default::default()};

        assert_eq!(player.can_pong(Tile::Wan(2)), vec![Meld::Pong(Tile::Wan(2))]);

        player.skipped_tile(Tile::Wan(2));

        assert_eq!(player.can_pong(Tile::Wan(2)), vec![]);
    }

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

    #[test]
    fn test_can_pong() {
        let hand_before_pong = HashMap::from([
            (Tile::Tong(1), 3),
            (Tile::Dragon(DragonType::Zhong), 2),
            (Tile::Hua(HuaType::RedThree), 1),
        ]);

        let player = Player {
            hand: hand_before_pong,
            ..Default::default()
        };

        // can only pong with 2 or more identical tiles
        let test_cases = vec![
            (Tile::Tong(1), vec![Meld::Pong(Tile::Tong(1))]),
            (
                Tile::Dragon(DragonType::Zhong),
                vec![Meld::Pong(Tile::Dragon(DragonType::Zhong))],
            ),
            (Tile::Hua(HuaType::RedThree), vec![]),
            (Tile::Tong(9), vec![]),
        ];

        for (tile, melds) in test_cases {
            assert_eq!(player.can_pong(tile), melds);
        }
    }

    #[test]
    fn test_can_gang() {
        let hand_before_gang =
            HashMap::from([(Tile::Tong(1), 3), (Tile::Wan(2), 2), (Tile::Suo(3), 1)]);

        let player = Player {
            hand: hand_before_gang,
            ..Default::default()
        };

        // can only gang with 3 identical tiles in hand
        let test_cases = vec![
            (Tile::Tong(1), vec![Meld::Gang(Tile::Tong(1))]),
            (Tile::Wan(2), vec![]),
            (Tile::Suo(3), vec![]),
            (Tile::Tong(9), vec![]),
        ];

        for (tile, melds) in test_cases {
            assert_eq!(player.can_gang(tile), melds);
        }
    }

    #[test]
    fn test_can_angang() {
        let hand_before_angang = HashMap::from([(Tile::Wan(2), 3), (Tile::Suo(3), 2)]);

        let mut player = Player {
            hand: hand_before_angang,
            ..Default::default()
        };

        // can only angang with 4 identical tiles in hand
        assert_eq!(player.can_angang(), vec![]);

        player.draw(Tile::Wan(2));

        assert_eq!(player.can_angang(), vec![Meld::AnGang(Tile::Wan(2))]);
    }
}

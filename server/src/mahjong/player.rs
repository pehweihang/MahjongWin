use std::collections::{HashMap, HashSet};

use thiserror::Error;

use super::{
    meld::{AnGang, Chi, Gang, Meld, Pong},
    tile::Tile,
};

#[derive(Debug, Error)]
#[error("Player tiles not able to perform action: {0}")]
pub struct IllegalMoveError(Action);

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
    /// Place a tile into the player's hand
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
    /// from a discarded tile of another player.
    /// Returns if the player is able 'chi' tile with tile_a and tile_b.
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'chi' can be performed on
    /// * `tile_a` - first tile to check in player's hand
    /// * `tile_b` - second tile to check in player's hand
    fn can_chi(&self, tile_a: &Tile, tile_b: &Tile, tile: &Tile) -> bool {
        if self.skipped_tiles.contains(tile) {
            return false;
        }

        matches!(
            (
                self.hand.contains_key(tile_a),
                self.hand.contains_key(tile_b)
            ),
            (true, true)
        )
    }

    /// A player can perform the action 'pong' if they can form a sequence of 3 identical tiles
    /// from a discarded tile of another player.
    /// Returns if the player is able to 'pong' the tile
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'pong' can be performed on
    fn can_pong(&self, tile: &Tile) -> bool {
        if self.skipped_tiles.contains(tile) {
            return false;
        }

        matches!(self.hand.get(tile), Some(x) if x >= &2)
    }

    /// A player can perform the action 'gang' if they can form a sequence of 4 identical tiles
    /// from a discarded tile of another player
    /// Returns if the player is able to 'gang' the tile
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'gang' can be performed on
    fn can_gang(&self, tile: &Tile) -> bool {
        matches!(self.hand.get(tile), Some(3))
    }

    /// A player can perform the action 'angang' if they can form a sequence of 4 identical tiles from their hand
    ///
    /// # Arguments
    ///
    /// * `tile` - the tile to check if 'angang' can be performed on
    fn can_angang(&self, tile: &Tile) -> bool {
        matches!(self.hand.get(tile), Some(4))
    }

    /// Returns all possible 'chi's a player can do from a tile.
    ///
    /// # Arguments
    ///
    /// * `tile` - tile to check
    pub fn get_chi(&self, tile: &Tile) -> Vec<Meld> {
        let mut possible_melds = vec![];

        if let Tile::Wan(_) | Tile::Suo(_) | Tile::Tong(_) = tile {
            let tile_as_int: i8 = tile.clone().into();

            // check all possible 'chi's for the tile
            for (a, b) in &[(-2, -1), (-1, 1), (1, 2)] {
                let tile_a = (tile_as_int + a).try_into().unwrap();
                let tile_b = (tile_as_int + b).try_into().unwrap();

                if self.can_chi(&tile_a, &tile_b, tile) {
                    if let Ok(chi) = Chi::new(tile_a, tile_b, tile.clone()) {
                        possible_melds.push(Meld::Chi(chi));
                    }
                }
            }
        };
        possible_melds
    }

    /// Returns all possible 'pong's a player can do from a tile.
    ///
    /// # Arguments
    ///
    /// * `tile` - tile to check
    pub fn get_pong(&self, tile: &Tile) -> Vec<Meld> {
        let mut possible_melds = vec![];

        if self.can_pong(tile) {
            if let Ok(pong) = Pong::new(tile.clone()) {
                possible_melds.push(Meld::Pong(pong));
            }
        }

        possible_melds
    }

    /// Returns all possible 'gang's a player can do from a tile.
    ///
    /// # Arguments
    ///
    /// * `tile` - tile to check
    pub fn get_gang(&self, tile: &Tile) -> Vec<Meld> {
        let mut possible_melds = vec![];

        if self.can_gang(tile) {
            if let Ok(gang) = Gang::new(tile.clone()) {
                possible_melds.push(Meld::Gang(gang));
            }
        }

        possible_melds
    }

    /// Returns all possible 'gang's a player can do from a tile.
    pub fn get_angang(&self) -> Vec<Meld> {
        let mut possible_melds = vec![];

        for tile in self.hand.keys() {
            if self.can_angang(tile) {
                if let Ok(angang) = AnGang::new(tile.clone()) {
                    possible_melds.push(Meld::AnGang(angang));
                }
            }
        }

        possible_melds
    }

    /// Perform the action 'chi'. After 'chi' the player will meld the set of tiles 'chi' is
    /// performed on.
    ///
    /// # Arguments
    ///
    /// * `chi` - Set of tiles to perform chi on.
    pub fn chi(&mut self, chi: Chi) -> Result<(), IllegalMoveError> {
        let tile_a: &Tile = chi.get_0();
        let tile_b: &Tile = chi.get_1();
        let tile_c: &Tile = chi.get_2();

        if self.can_chi(tile_a, tile_b, tile_c) {
            self.remove_tiles_from_hand(tile_a, 1);
            self.remove_tiles_from_hand(tile_b, 1);
            self.melds.push(Meld::Chi(chi));
            Ok(())
        } else {
            Err(IllegalMoveError(Action::Meld(Meld::Chi(chi))))
        }
    }

    /// Perform the action 'pong'. After 'pong' the player will meld the set of tiles 'pong' is
    /// performed on.
    ///
    /// # Arguments
    ///
    /// * `pong` - Set of tiles to perform 'pong' on.
    pub fn pong(&mut self, pong: Pong) -> Result<(), IllegalMoveError> {
        let tile: &Tile = pong.get_0();

        if self.can_pong(tile) {
            self.remove_tiles_from_hand(tile, 2);
            self.melds.push(Meld::Pong(pong));
            Ok(())
        } else {
            Err(IllegalMoveError(Action::Meld(Meld::Pong(pong))))
        }
    }

    /// Perform the action 'gang'. After 'gang' the player will meld the set of tiles 'gang' is
    /// performed on.
    ///
    /// # Arguments
    ///
    /// * `gang` - Set of tiles to perform 'gang' on.
    pub fn gang(&mut self, gang: Gang) -> Result<(), IllegalMoveError> {
        let tile: &Tile = gang.get_0();

        if self.can_gang(tile) {
            self.remove_tiles_from_hand(tile, 3);
            self.melds.push(Meld::Gang(gang));
            Ok(())
        } else {
            Err(IllegalMoveError(Action::Meld(Meld::Gang(gang))))
        }
    }

    /// Perform the action 'angang'. After 'angang' the player will meld the set of tiles 'angang'
    /// is performed on
    ///
    /// # Arguments
    ///
    /// * `gang` - Set of tiles to perform 'gang' on.
    pub fn angang(&mut self, angang: AnGang) -> Result<(), IllegalMoveError> {
        let tile: &Tile = angang.get_0();

        if self.can_angang(tile) {
            self.remove_tiles_from_hand(tile, 4);
            self.melds.push(Meld::AnGang(angang));
            Ok(())
        } else {
            Err(IllegalMoveError(Action::Meld(Meld::AnGang(angang))))
        }
    }

    /// Utility function to remove tiles from player's hand
    ///
    /// # Arguments
    ///
    /// * `tile` - The tile to remove from player's hand
    /// * `amount_to_remove` - The amount to remove of the specified tile
    fn remove_tiles_from_hand(&mut self, tile: &Tile, amount_to_remove: u8) -> Result<(), String> {
        if let Some(value) = self.hand.get_mut(tile) {
            if *value >= amount_to_remove {
                *value -= amount_to_remove;
                Ok(())
            } else {
                // TODO implement error handling
                Err("Not enough tiles".to_string())
            }
        } else {
            // TODO implement error handling
            Err("Not enough tiles".to_string())
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Hu,
    ZiMuo,
    Meld(Meld),
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use claims::{assert_err, assert_ok};

    use crate::mahjong::{
        meld::{AnGang, Chi, Gang, Meld, Pong},
        tile::{DragonType, HuaType, Tile},
    };

    use super::Player;

    #[test]
    fn test_draw_tile() {
        let mut player = Player {
            ..Default::default()
        };
        player.draw(Tile::Wan(1));
        assert_eq!(player.hand, HashMap::from([(Tile::Wan(1), 1)]));
    }

    #[test]
    fn test_skipped_tile() {
        let hand = HashMap::from([(Tile::Wan(2), 2)]);
        let mut player = Player {
            hand,
            ..Default::default()
        };

        assert_eq!(
            player.get_pong(&Tile::Wan(2)),
            vec![Meld::Pong(Pong::new(Tile::Wan(2)).unwrap())]
        );

        player.skipped_tile(Tile::Wan(2));

        assert_eq!(player.get_pong(&Tile::Wan(2)), vec![]);
    }

    #[test]
    fn test_can_chi() {
        let hand = HashMap::from([
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
            hand,
            ..Default::default()
        };

        let test_cases = vec![
            // chi right
            ((Tile::Wan(1), Tile::Wan(2), Tile::Wan(3)), true),
            // chi left
            ((Tile::Wan(8), Tile::Wan(9), Tile::Wan(7)), true),
            // chi middle
            ((Tile::Suo(1), Tile::Suo(3), Tile::Suo(2)), true),
            // chi multiple
            ((Tile::Suo(6), Tile::Suo(8), Tile::Suo(7)), true),
            ((Tile::Suo(8), Tile::Suo(9), Tile::Suo(7)), true),
            // cannot chi
            ((Tile::Tong(1), Tile::Tong(2), Tile::Tong(3)), false),
        ];

        for ((t, ta, tb), res) in test_cases {
            assert_eq!(player.can_chi(&t, &ta, &tb), res);
        }
    }

    #[test]
    fn test_get_chi() {
        let hand = HashMap::from([
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
            hand,
            ..Default::default()
        };

        // (tile to check, possible chi's)
        let test_cases = vec![
            (
                // chi right
                Tile::Wan(3),
                vec![Meld::Chi(
                    Chi::new(Tile::Wan(1), Tile::Wan(2), Tile::Wan(3)).unwrap(),
                )],
            ),
            // chi left
            (
                Tile::Wan(7),
                vec![Meld::Chi(
                    Chi::new(Tile::Wan(8), Tile::Wan(9), Tile::Wan(7)).unwrap(),
                )],
            ),
            // chi middle
            (
                Tile::Suo(2),
                vec![Meld::Chi(
                    Chi::new(Tile::Suo(1), Tile::Suo(3), Tile::Suo(2)).unwrap(),
                )],
            ),
            // chi multiple
            (
                Tile::Suo(7),
                vec![
                    Meld::Chi(Chi::new(Tile::Suo(6), Tile::Suo(8), Tile::Suo(7)).unwrap()),
                    Meld::Chi(Chi::new(Tile::Suo(8), Tile::Suo(9), Tile::Suo(7)).unwrap()),
                ],
            ),
            // getnot chi
            (Tile::Tong(1), vec![]),
        ];

        for (tile, melds) in test_cases {
            assert_eq!(player.get_chi(&tile), melds);
        }
    }

    #[test]
    fn test_can_pong() {
        let hand = HashMap::from([
            (Tile::Tong(1), 3),
            (Tile::Dragon(DragonType::Zhong), 2),
            (Tile::Suo(3), 1),
        ]);

        let player = Player {
            hand,
            ..Default::default()
        };

        // get only pong with 2 or more identical tiles
        let test_cases = vec![
            (Tile::Tong(1), true),
            (Tile::Dragon(DragonType::Zhong), true),
            (Tile::Suo(3), false),
            (Tile::Tong(9), false),
        ];

        for (tile, res) in test_cases {
            assert_eq!(player.can_pong(&tile), res);
        }
    }

    #[test]
    fn test_get_pong() {
        let hand_before_pong = HashMap::from([
            (Tile::Tong(1), 3),
            (Tile::Dragon(DragonType::Zhong), 2),
            (Tile::Suo(2), 1),
        ]);

        let player = Player {
            hand: hand_before_pong,
            ..Default::default()
        };

        // get only pong with 2 or more identical tiles
        let test_cases = vec![
            (
                Tile::Tong(1),
                vec![Meld::Pong(Pong::new(Tile::Tong(1)).unwrap())],
            ),
            (
                Tile::Dragon(DragonType::Zhong),
                vec![Meld::Pong(
                    Pong::new(Tile::Dragon(DragonType::Zhong)).unwrap(),
                )],
            ),
            (Tile::Suo(2), vec![]),
            (Tile::Tong(9), vec![]),
        ];

        for (tile, melds) in test_cases {
            assert_eq!(player.get_pong(&tile), melds);
        }
    }

    #[test]
    fn test_can_gang() {
        let hand = HashMap::from([(Tile::Tong(1), 3), (Tile::Wan(2), 2), (Tile::Suo(3), 1)]);

        let player = Player {
            hand,
            ..Default::default()
        };

        // get only gang with 3 identical tiles in hand
        let test_cases = vec![
            (Tile::Tong(1), true),
            (Tile::Wan(2), false),
            (Tile::Suo(3), false),
            (Tile::Tong(9), false),
        ];

        for (tile, res) in test_cases {
            assert_eq!(player.can_gang(&tile), res);
        }
    }

    #[test]
    fn test_get_gang() {
        let hand = HashMap::from([(Tile::Tong(1), 3), (Tile::Wan(2), 2), (Tile::Suo(3), 1)]);

        let player = Player {
            hand,
            ..Default::default()
        };

        // get only gang with 3 identical tiles in hand
        let test_cases = vec![
            (
                Tile::Tong(1),
                vec![Meld::Gang(Gang::new(Tile::Tong(1)).unwrap())],
            ),
            (Tile::Wan(2), vec![]),
            (Tile::Suo(3), vec![]),
            (Tile::Tong(9), vec![]),
        ];

        for (tile, melds) in test_cases {
            assert_eq!(player.get_gang(&tile), melds);
        }
    }

    #[test]
    fn test_can_angang() {
        let hand = HashMap::from([(Tile::Wan(2), 3), (Tile::Suo(3), 2)]);

        let mut player = Player {
            hand,
            ..Default::default()
        };

        // get only angang with 4 identical tiles in hand
        assert!(!player.can_angang(&Tile::Wan(2)));
        assert!(!player.can_angang(&Tile::Suo(3)));

        player.draw(Tile::Wan(2));

        assert!(player.can_angang(&Tile::Wan(2)));
        assert!(!player.can_angang(&Tile::Suo(3)));
    }

    #[test]
    fn test_get_angang() {
        let hand = HashMap::from([(Tile::Wan(2), 3), (Tile::Suo(3), 2)]);

        let mut player = Player {
            hand,
            ..Default::default()
        };

        // get only angang with 4 identical tiles in hand
        assert_eq!(player.get_angang(), vec![]);

        player.draw(Tile::Wan(2));

        assert_eq!(
            player.get_angang(),
            vec![Meld::AnGang(AnGang::new(Tile::Wan(2)).unwrap())]
        );
    }

    #[test]
    fn test_chi() {
        let hand = HashMap::from([(Tile::Wan(3), 1), (Tile::Wan(4), 1)]);
        let mut player = Player {
            hand,
            ..Default::default()
        };
        let chi_err = Chi::new(Tile::Wan(4), Tile::Wan(5), Tile::Wan(6)).unwrap();
        assert_err!(player.chi(chi_err));

        let chi_ok = Chi::new(Tile::Wan(3), Tile::Wan(4), Tile::Wan(2)).unwrap();
        assert_ok!(player.chi(chi_ok));
        assert!(player.melds.contains(&Meld::Chi(chi_ok)));
    }

    #[test]
    fn test_pong() {
        let hand = HashMap::from([(Tile::Wan(1), 2)]);
        let mut player = Player {
            hand,
            ..Default::default()
        };

        let pong_err = Pong::new(Tile::Wan(2)).unwrap();
        assert_err!(player.pong(pong_err));

        let pong_ok = Pong::new(Tile::Wan(1)).unwrap();
        assert_ok!(player.pong(pong_ok));
        assert!(player.melds.contains(&Meld::Pong(pong_ok)));
    }

    #[test]
    fn test_gang() {
        let hand = HashMap::from([(Tile::Wan(1), 3)]);
        let mut player = Player {
            hand,
            ..Default::default()
        };

        let gang_err = Gang::new(Tile::Wan(2)).unwrap();
        assert_err!(player.gang(gang_err));

        let gang_ok = Gang::new(Tile::Wan(1)).unwrap();
        assert_ok!(player.gang(gang_ok));
        assert!(player.melds.contains(&Meld::Gang(gang_ok)));
    }

    #[test]
    fn test_angang() {
        let hand = HashMap::from([(Tile::Wan(1), 3)]);
        let mut player = Player {
            hand,
            ..Default::default()
        };

        let angang_err = AnGang::new(Tile::Wan(1)).unwrap();
        assert_err!(player.angang(angang_err));

        player.draw(Tile::Wan(1));
        let angang_ok = AnGang::new(Tile::Wan(1)).unwrap();
        assert_ok!(player.angang(angang_ok));
        assert!(player.melds.contains(&Meld::AnGang(angang_ok)));
    }
}

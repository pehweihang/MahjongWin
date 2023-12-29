use std::collections::{HashMap, HashSet};

use crate::{
    error::MahjongError,
    meld::{Meld, MeldType},
    tile::Tile,
};

#[derive(Debug, Default, Clone)]
pub struct ConcealedTiles(HashMap<Tile, u8>);

impl std::ops::Deref for ConcealedTiles {
    type Target = HashMap<Tile, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ConcealedTiles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ConcealedTiles {
    pub fn remove_n(&mut self, tile: &Tile, n: u8) -> Result<(), MahjongError> {
        match self.get_mut(tile) {
            Some(c) if *c > n => {
                *c -= n;
                Ok(())
            }
            Some(c) if *c == n => {
                self.remove(tile);
                Ok(())
            }
            Some(_) => Err(MahjongError::TileNotInHandFoundError(*tile)),
            None => Err(MahjongError::TileNotInHandFoundError(*tile)),
        }
    }

    pub fn add_n(&mut self, tile: &Tile, n: u8) {
        *self.entry(*tile).or_insert(0) += n;
    }

}

#[derive(Debug, Default)]
pub struct Hand {
    concealed: ConcealedTiles,
    melds: Vec<Meld>,
    bonus: HashSet<Tile>,
    seen: HashSet<Tile>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn draw(&mut self, tile: &Tile) {
        match tile.is_playable() {
            true => {
                self.concealed.add_n(tile, 1);
            }
            false => {
                self.bonus.insert(*tile);
            }
        };
        self.seen.clear();
    }

    pub fn discard(&mut self, tile: &Tile) -> Result<(), MahjongError> {
        self.concealed.remove_n(tile, 1)?;
        self.seen.insert(tile.to_owned());
        Ok(())
    }

    pub fn get_melds(&self, tile: &Tile) -> Result<Vec<Meld>, MahjongError> {
        if !tile.is_playable() {
            return Err(MahjongError::TileNotPlayableError(tile.suit()));
        }
        let mut poss_melds = vec![];
        if let Some(num) = self.concealed.get(tile) {
            if *num >= 2 {
                poss_melds.push(Meld::new(
                    vec![tile.to_owned(), tile.to_owned()],
                    Some(tile.to_owned()),
                    MeldType::Pong,
                )?);
            }
            if *num >= 3 {
                poss_melds.push(Meld::new(
                    vec![tile.to_owned(), tile.to_owned(), tile.to_owned()],
                    Some(tile.to_owned()),
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
                let mut tiles = [t1.to_owned(), t2.to_owned(), tile.to_owned()];
                tiles.sort();
                if self.concealed.contains_key(t1) && self.concealed.contains_key(t2) {
                    poss_melds.push(Meld::new(
                        vec![t1.to_owned(), t2.to_owned()],
                        Some(tile.to_owned()),
                        MeldType::Chi,
                    )?);
                }
            }
        }
        Ok(poss_melds)
    }

    pub fn meld(&mut self, meld: Meld) -> Result<(), MahjongError> {
        let mut map = ConcealedTiles {
            ..Default::default()
        };
        for tile in meld.tiles() {
            *map.entry(tile.to_owned()).or_insert(0_u8) += 1;
        }
        for (tile, count) in map.iter() {
            self.concealed.remove_n(tile, *count)?;
        }
        self.melds.push(meld);
        Ok(())
    }

    pub fn get_angangs(&self) -> Vec<Meld> {
        let mut melds = Vec::new();
        for (tile, count) in self.concealed.iter() {
            if *count == 4 {
                melds.push(
                    Meld::new(
                        vec![
                            tile.to_owned(),
                            tile.to_owned(),
                            tile.to_owned(),
                            tile.to_owned(),
                        ],
                        None,
                        MeldType::AnGang,
                    )
                    .unwrap(),
                );
            }
        }
        melds
    }

    pub fn concealed(&self) -> &ConcealedTiles {
        &self.concealed
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use claim::{assert_err, assert_ok_eq};

    use crate::{
        meld::{Meld, MeldType},
        tile::{Tile, TileValue},
    };

    use super::Hand;

    #[test]
    fn test_draw_tile_ok() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::One));
        assert!(hand
            .concealed
            .eq(&HashMap::from([(Tile::Wan(TileValue::One), 1)])));
        hand.draw(&Tile::Wan(TileValue::One));
        assert!(hand
            .concealed
            .eq(&HashMap::from([(Tile::Wan(TileValue::One), 2)])));
    }

    #[test]
    fn test_no_seen_tiles_after_draw() {
        let mut hand = Hand::new();
        hand.seen.insert(Tile::Wan(TileValue::One));
        hand.draw(&Tile::Suo(TileValue::One));
        assert_eq!(hand.seen.len(), 0);
    }

    #[test]
    fn test_discard_tile_ok() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::One));
        hand.draw(&Tile::Wan(TileValue::One));
        hand.discard(&Tile::Wan(TileValue::One)).unwrap();
        assert_eq!(hand.concealed.0, HashMap::from([(Tile::Wan(TileValue::One), 1)]));
        hand.discard(&Tile::Wan(TileValue::One)).unwrap();
        assert!(hand.concealed.eq(&HashMap::from([])));
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
        assert!(hand.seen.contains(&Tile::Wan(TileValue::One)));
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

        let correct_melds = vec![
            Meld::new(
                vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
                Some(Tile::Wan(TileValue::Four)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Three), Tile::Wan(TileValue::Five)],
                Some(Tile::Wan(TileValue::Four)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Five), Tile::Wan(TileValue::Six)],
                Some(Tile::Wan(TileValue::Four)),
                MeldType::Chi,
            )
            .unwrap(),
            Meld::new(
                vec![Tile::Wan(TileValue::Four), Tile::Wan(TileValue::Four)],
                Some(Tile::Wan(TileValue::Four)),
                MeldType::Pong,
            )
            .unwrap(),
            Meld::new(
                vec![
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                    Tile::Wan(TileValue::Four),
                ],
                Some(Tile::Wan(TileValue::Four)),
                MeldType::Gang,
            )
            .unwrap(),
        ];
        assert!(hand
            .get_melds(&Tile::Wan(TileValue::Four))
            .unwrap()
            .iter()
            .all(|m| correct_melds.contains(m)));
    }

    #[test]
    fn test_meld_ok() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Three));
        let meld = Meld::new(
            vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
            Some(Tile::Wan(TileValue::Four)),
            MeldType::Chi,
        )
        .unwrap();

        assert_ok_eq!(hand.meld(meld.clone()), ());
        assert_eq!(hand.melds, vec![meld]);
        assert!(hand.concealed.eq(&HashMap::from([])));
    }

    #[test]
    fn test_meld_fail() {
        let mut hand = Hand::new();
        let meld = Meld::new(
            vec![Tile::Wan(TileValue::Two), Tile::Wan(TileValue::Three)],
            Some(Tile::Wan(TileValue::Four)),
            MeldType::Chi,
        )
        .unwrap();

        assert_err!(hand.meld(meld.clone()));
    }

    #[test]
    fn test_get_angang() {
        let mut hand = Hand::new();
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Two));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Three));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Four));
        hand.draw(&Tile::Wan(TileValue::Four));
        let correct_melds = vec![
            Meld::new(vec![Tile::Wan(TileValue::Two); 4], None, MeldType::AnGang).unwrap(),
            Meld::new(vec![Tile::Wan(TileValue::Three); 4], None, MeldType::AnGang).unwrap(),
        ];
        assert!(hand.get_angangs().iter().all(|m| correct_melds.contains(m)));
    }
}

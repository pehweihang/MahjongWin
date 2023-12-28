use crate::{
    error::MahjongError,
    tile::{Suit, Tile},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Meld {
    tiles: Vec<Tile>,
    discarded_tile: Option<Tile>,
    meld_type: MeldType,
    suit: Suit,
}

impl Meld {
    pub fn new(
        mut tiles: Vec<Tile>,
        discarded_tile: Option<Tile>,
        meld_type: MeldType,
    ) -> Result<Self, MahjongError> {
        tiles.sort();
        let suit = tiles
            .first()
            .ok_or(MahjongError::InvalidMeldError(
                meld_type,
                tiles.clone(),
                discarded_tile,
            ))?
            .suit();
        if !tiles.first().unwrap().is_playable() {
            return Err(MahjongError::InvalidMeldError(
                meld_type,
                tiles,
                discarded_tile,
            ));
        }

        match discarded_tile {
            Some(discarded_tile) => {
                let mut all_tiles = tiles.clone();
                all_tiles.push(discarded_tile);
                all_tiles.sort();
                match meld_type {
                    MeldType::Chi => {
                        if all_tiles.len() != 3
                            || !all_tiles.windows(2).all(|w| match w[0].next() {
                                Some(t) => t == w[1],
                                None => false,
                            })
                        {
                            return Err(MahjongError::InvalidMeldError(
                                meld_type,
                                tiles,
                                Some(discarded_tile),
                            ));
                        }
                    }
                    MeldType::Pong => {
                        if all_tiles.len() != 3 || !all_tiles.windows(2).all(|w| w[0] == w[1]) {
                            return Err(MahjongError::InvalidMeldError(
                                meld_type,
                                tiles,
                                Some(discarded_tile),
                            ));
                        }
                    }
                    MeldType::Gang => {
                        if all_tiles.len() != 4 || !all_tiles.windows(2).all(|w| w[0] == w[1]) {
                            return Err(MahjongError::InvalidMeldError(
                                meld_type,
                                tiles,
                                Some(discarded_tile),
                            ));
                        }
                    }
                    _ => {
                        return Err(MahjongError::InvalidMeldError(
                            meld_type,
                            tiles,
                            Some(discarded_tile),
                        ));
                    }
                }
            }
            None => match meld_type {
                MeldType::Eye => {
                    if tiles.len() != 2 || tiles[0] != tiles[1] {
                        return Err(MahjongError::InvalidMeldError(
                            meld_type,
                            tiles,
                            discarded_tile,
                        ));
                    }
                }
                MeldType::AnGang => {}
                _ => {
                    return Err(MahjongError::InvalidMeldError(
                        meld_type,
                        tiles,
                        discarded_tile,
                    ));
                }
            },
        }

        Ok(Self {
            tiles,
            discarded_tile,
            meld_type,
            suit,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeldType {
    Chi,
    Pong,
    Gang,
    AnGang,
    Eye,
}

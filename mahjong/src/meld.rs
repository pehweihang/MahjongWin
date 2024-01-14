use crate::{
    error::MahjongError,
    tile::{Suit, Tile},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        let mut all_tiles = tiles.clone();
        if let Some(discarded) = discarded_tile {
            all_tiles.push(discarded)
        }
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
                        discarded_tile,
                    ));
                }
            }
            MeldType::Pong => {
                if all_tiles.len() != 3 || !all_tiles.windows(2).all(|w| w[0] == w[1]) {
                    return Err(MahjongError::InvalidMeldError(
                        meld_type,
                        tiles,
                        discarded_tile,
                    ));
                }
            }
            MeldType::Gang | MeldType::AnGang => {
                if all_tiles.len() != 4 || !all_tiles.windows(2).all(|w| w[0] == w[1]) {
                    return Err(MahjongError::InvalidMeldError(
                        meld_type,
                        tiles,
                        discarded_tile,
                    ));
                }
            }
            MeldType::Eye => {
                if tiles.len() != 2 || tiles[0] != tiles[1] {
                    return Err(MahjongError::InvalidMeldError(
                        meld_type,
                        tiles,
                        discarded_tile,
                    ));
                }
            }
        }

        Ok(Self {
            tiles,
            discarded_tile,
            meld_type,
            suit,
        })
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn meld_type(&self) -> &MeldType {
        &self.meld_type
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

use crate::{
    error::MahjongError,
    tile::{Suit, Tile},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Meld {
    tiles: Vec<Tile>,
    meld_type: MeldType,
    suit: Suit,
}

impl Meld {
    pub fn new(mut tiles: Vec<Tile>, meld_type: MeldType) -> Result<Self, MahjongError> {
        let suit = tiles
            .first()
            .ok_or(MahjongError::InvalidMeldError(tiles.clone()))?
            .suit();
        if suit == Suit::Animal || suit == Suit::Flower {
            return Err(MahjongError::InvalidMeldError(tiles));
        }
        match meld_type {
            MeldType::Chi => {
                if (suit != Suit::Wan && suit != Suit::Suo && suit != Suit::Tong)
                    || tiles.len() != 3
                    || !tiles.iter().all(|t| t.suit() == suit)
                {
                    return Err(MahjongError::InvalidMeldError(tiles));
                }

                tiles.sort();
                if !tiles.windows(2).all(|w| {
                    if let Some(x) = w[0].next() {
                        x == w[1] && x.suit() == suit
                    } else {
                        false
                    }
                }) {
                    return Err(MahjongError::InvalidMeldError(tiles));
                }
            }
            MeldType::Pong => {
                if tiles.len() != 3 {
                    return Err(MahjongError::InvalidMeldError(tiles));
                }
                if !tiles.windows(2).all(|w| w[0] == w[1]) {
                    return Err(MahjongError::InvalidMeldError(tiles));
                }
            }
            MeldType::Gang | MeldType::AnGang => {
                if tiles.len() != 4 {
                    return Err(MahjongError::InvalidMeldError(tiles));
                }
                if !tiles.windows(2).all(|w| w[0] == w[1]) {
                    return Err(MahjongError::InvalidMeldError(tiles));
                }
            }
        }

        Ok(Self {
            tiles,
            meld_type,
            suit,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeldType {
    Chi,
    Pong,
    Gang,
    AnGang,
}

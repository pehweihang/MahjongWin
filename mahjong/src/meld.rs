use crate::tile::{Suit, Tile};

#[derive(Debug)]
pub struct Meld {
    pub tiles: Vec<Tile>,
    pub meld_type: MeldType,
    pub suit: Suit,
}

impl Meld {
    pub fn new(mut tiles: Vec<Tile>, meld_type: MeldType) -> Result<Self, String> {
        let suit = tiles.first().ok_or("No tiles to meld.")?.suit();
        if suit == Suit::Animal || suit == Suit::Flower {
            return Err(format!("Cannot meld tile of suit {:?}", suit));
        }
        match meld_type {
            MeldType::Chi => {
                if suit != Suit::Wan || suit != Suit::Suo || suit != Suit::Tong {
                    return Err(format!("Cannot Chi suit {:?}", suit));
                }
                if tiles.len() != 3 {
                    return Err(format!("Cannot Chi {} tiles", tiles.len()));
                }
                tiles.sort();
                if !tiles.windows(2).all(|w| {
                    if let Some(x) = w[0].next() {
                        x == w[1]
                    } else {
                        false
                    }
                }) {
                    return Err(format!("Tiles {:?} is not a sequence", tiles));
                }
            }
            MeldType::Pong => {
                if tiles.len() != 3 {
                    return Err(format!("Cannot Pong {} tiles", tiles.len()));
                }
                if !tiles.windows(2).all(|w| w[0] == w[1]) {
                    return Err(format!("Can only Gang sets of same tiles, got {:?}", tiles));
                }
            }
            MeldType::Gang | MeldType::AnGang => {
                if tiles.len() != 4 {
                    return Err(format!("Cannot Gang {} tiles", tiles.len()));
                }
                if !tiles.windows(2).all(|w| w[0] == w[1]) {
                    return Err(format!("Can only Gang sets of same tiles, got {:?}", tiles));
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

#[derive(Debug)]
pub enum MeldType {
    Chi,
    Pong,
    Gang,
    AnGang,
}

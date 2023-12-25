use thiserror::Error;

use crate::tile::{Tile, Suit};

#[derive(Error, Debug)]
pub enum MahjongError {
    #[error("Tile {0:?} not in hand")]
    TileNotFoundError(Tile),
    #[error("Tile with suit {0:?} is not playable")]
    TileNotPlayableError(Suit),
    #[error("Cannot create meld from {0:?}")]
    InvalidMeldError(Vec<Tile>)
}

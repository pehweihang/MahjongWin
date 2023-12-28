use thiserror::Error;

use crate::{tile::{Tile, Suit}, meld::MeldType};

#[derive(Error, Debug)]
pub enum MahjongError {
    #[error("Tile {0:?} not in hand")]
    TileNotInHandFoundError(Tile),
    #[error("Tile with suit {0:?} is not playable")]
    TileNotPlayableError(Suit),
    #[error("Cannot create meld type {0:?} from {1:?} and {2:?}")]
    InvalidMeldError(MeldType, Vec<Tile>, Option<Tile>)
}

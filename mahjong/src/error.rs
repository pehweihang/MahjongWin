use thiserror::Error;

use crate::tile::Tile;

#[derive(Error, Debug)]
pub enum MahjongError {
    #[error("Tile {0:?} not in hand")]
    TileNotFoundError(Tile),
}

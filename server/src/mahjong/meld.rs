use std::mem::discriminant;

use thiserror::Error;

use super::tile::Tile;

#[derive(Debug, PartialEq, Eq)]
pub enum Meld {
    Chi(Chi),
    Pong(Pong),
    Gang(Gang),
    AnGang(AnGang),
}

fn is_value_suit(tile: &Tile) -> bool {
    matches!(tile, Tile::Wan(_) | Tile::Suo(_) | Tile::Tong(_))
}

fn is_playable_tile(tile: &Tile) -> bool {
    !matches!(tile, Tile::Hua(_) | Tile::Animal(_))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Chi(Tile, Tile, Tile);

impl Chi {
    pub fn new(x: Tile, y: Tile, z: Tile) -> Result<Self, IllegalChiError> {
        for tile in [&x, &y, &z] {
            if !is_value_suit(tile) {
                return Err(IllegalChiError::IllegalTileError(tile.clone()));
            }
        }
        if !(discriminant(&x) == discriminant(&y) && discriminant(&x) == discriminant(&z)) {
            return Err(IllegalChiError::NotSameSuitError(x, y, z));
        }

        let mut seq: [i8; 3] = [x.clone().into(), y.clone().into(), z.clone().into()];
        seq.sort();
        if !(seq[0] == seq[1] - 1 && seq[0] == seq[2] - 2) {
            return Err(IllegalChiError::NotASequenceError(x, y, z));
        }

        Ok(Self(x, y, z))
    }

    pub fn get_0(&self) -> &Tile {
        &self.0
    }

    pub fn get_1(&self) -> &Tile {
        &self.1
    }

    pub fn get_2(&self) -> &Tile {
        &self.2
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pong(Tile);

impl Pong {
    pub fn new(tile: Tile) -> Result<Self, IllegalPongError> {
        match is_playable_tile(&tile) {
            true => Ok(Self(tile)),
            false => Err(IllegalPongError::IllegalTileError(tile)),
        }
    }

    pub fn get_0(&self) -> &Tile {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Gang(Tile);

impl Gang {
    pub fn new(tile: Tile) -> Result<Self, IllegalGangError> {
        match is_playable_tile(&tile) {
            true => Ok(Self(tile)),
            false => Err(IllegalGangError::IllegalTileError(tile)),
        }
    }
    pub fn get_0(&self) -> &Tile {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AnGang(Tile);

impl AnGang {
    pub fn new(tile: Tile) -> Result<Self, IllegalAnGangError> {
        match is_playable_tile(&tile) {
            true => Ok(Self(tile)),
            false => Err(IllegalAnGangError::IllegalTileError(tile)),
        }
    }
    pub fn get_0(&self) -> &Tile {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum IllegalChiError {
    #[error("Illegal to chi tile: {0:?}. Chi can only be performed on Wan, Suo and Tong")]
    IllegalTileError(Tile),
    #[error("Tiles {0:?}, {1:?}, {2:?} are not a valid sequence")]
    NotASequenceError(Tile, Tile, Tile),
    #[error("Tiles {0:?}, {1:?}, {2:?} are not the same suit")]
    NotSameSuitError(Tile, Tile, Tile),
}

#[derive(Debug, Error)]
pub enum IllegalPongError {
    #[error(
        "Illegal to pong tile: {0:?}. Pong can only be performed on Wan, Suo, Tong, Feng, Dragon"
    )]
    IllegalTileError(Tile),
}

#[derive(Debug, Error)]
pub enum IllegalGangError {
    #[error(
        "Illegal to gang tile: {0:?}. Gang can only be performed on Wan, Suo, Tong, Feng, Dragon"
    )]
    IllegalTileError(Tile),
}

#[derive(Debug, Error)]
pub enum IllegalAnGangError {
    #[error("Illegal to angang tile: {0:?}. An gang can only be performed on Wan, Suo, Tong, Feng, Dragon")]
    IllegalTileError(Tile),
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use crate::mahjong::{
        meld::{AnGang, Chi, Gang, Pong},
        tile::{AnimalType, HuaType, Tile},
    };

    #[test]
    fn test_create_chi() {
        assert_err!(Chi::new(
            Tile::Wan(1),
            Tile::Wan(2),
            Tile::Animal(AnimalType::Cat)
        ));
        assert_err!(Chi::new(Tile::Wan(1), Tile::Wan(2), Tile::Tong(3)));
        assert_err!(Chi::new(Tile::Wan(1), Tile::Wan(2), Tile::Wan(4)));
        assert_ok!(Chi::new(Tile::Wan(1), Tile::Wan(2), Tile::Wan(3)));
    }

    #[test]
    fn test_create_pong() {
        assert_err!(Pong::new(Tile::Hua(HuaType::RedOne)));
        assert_ok!(Pong::new(Tile::Tong(1)));
    }

    #[test]
    fn test_create_gang() {
        assert_err!(Gang::new(Tile::Hua(HuaType::RedOne)));
        assert_ok!(Gang::new(Tile::Tong(1)));
    }

    #[test]
    fn test_create_angang() {
        assert_err!(AnGang::new(Tile::Hua(HuaType::RedOne)));
        assert_ok!(AnGang::new(Tile::Tong(1)));
    }
}

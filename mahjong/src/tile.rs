#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Wan(TileValue),
    Suo(TileValue),
    Tong(TileValue),
    Wind(Wind),
    Dragon(Dragon),
    Animal(Animal),
    Flower(Flower),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dragon {
    Zhong,
    Fa,
    Baiban,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Animal {
    Cat,
    Rat,
    Chicken,
    Centipede,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flower {
    Red(FlowerValue),
    Blue(FlowerValue),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlowerValue {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Wan,
    Suo,
    Tong,
    Wind,
    Dragon,
    Animal,
    Flower,
}

impl Tile {
    pub fn next(&self) -> Option<Tile> {
        match self {
            Tile::Wan(val) => Some(Tile::Wan(val.next()?)),
            Tile::Suo(val) => Some(Tile::Suo(val.next()?)),
            Tile::Tong(val) => Some(Tile::Tong(val.next()?)),
            _ => None,
        }
    }

    pub fn prev(&self) -> Option<Tile> {
        match self {
            Tile::Wan(val) => Some(Tile::Wan(val.prev()?)),
            Tile::Suo(val) => Some(Tile::Suo(val.prev()?)),
            Tile::Tong(val) => Some(Tile::Tong(val.prev()?)),
            _ => None,
        }
    }

    pub fn suit(&self) -> Suit {
        match self {
            Tile::Wan(_) => Suit::Wan,
            Tile::Suo(_) => Suit::Suo,
            Tile::Tong(_) => Suit::Tong,
            Tile::Wind(_) => Suit::Wind,
            Tile::Dragon(_) => Suit::Dragon,
            Tile::Animal(_) => Suit::Animal,
            Tile::Flower(_) => Suit::Flower,
        }
    }
}

impl TileValue {
    pub fn next(&self) -> Option<TileValue> {
        match self {
            TileValue::One => Some(TileValue::Two),
            TileValue::Two => Some(TileValue::Three),
            TileValue::Three => Some(TileValue::Four),
            TileValue::Four => Some(TileValue::Five),
            TileValue::Five => Some(TileValue::Six),
            TileValue::Six => Some(TileValue::Seven),
            TileValue::Seven => Some(TileValue::Eight),
            TileValue::Eight => Some(TileValue::Nine),
            TileValue::Nine => None,
        }
    }

    pub fn prev(&self) -> Option<TileValue> {
        match self {
            TileValue::One => None,
            TileValue::Two => Some(TileValue::One),
            TileValue::Three => Some(TileValue::Two),
            TileValue::Four => Some(TileValue::Three),
            TileValue::Five => Some(TileValue::Four),
            TileValue::Six => Some(TileValue::Five),
            TileValue::Seven => Some(TileValue::Six),
            TileValue::Eight => Some(TileValue::Seven),
            TileValue::Nine => Some(TileValue::Eight),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::Wind;

    use super::{Tile, TileValue};

    #[test]
    fn test_tile_next_ok() {
        let tile = Tile::Suo(TileValue::One);
        assert_eq!(tile.next(), Some(Tile::Suo(TileValue::Two)));
    }

    #[test]
    fn test_tile_next_out_of_bounds() {
        let tile = Tile::Suo(TileValue::Nine);
        assert_eq!(tile.next(), None);
    }

    #[test]
    fn test_tile_next_not_a_value_tile() {
        let tile = Tile::Wind(Wind::East);
        assert_eq!(tile.next(), None);
    }

    #[test]
    fn test_tile_prev_ok() {
        let tile = Tile::Suo(TileValue::Two);
        assert_eq!(tile.prev(), Some(Tile::Suo(TileValue::One)));
    }

    #[test]
    fn test_tile_prev_out_of_bounds() {
        let tile = Tile::Suo(TileValue::One);
        assert_eq!(tile.prev(), None);
    }

    #[test]
    fn test_tile_prev_not_a_value_tile() {
        let tile = Tile::Wind(Wind::East);
        assert_eq!(tile.prev(), None);
    }
}

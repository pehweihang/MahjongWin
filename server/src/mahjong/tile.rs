#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Tile {
    Wan(i8),
    Suo(i8),
    Tong(i8),
    Feng(FengType),
    Dragon(DragonType),
    Hua(HuaType),
    Animal(AnimalType),
}

impl TryFrom<i8> for Tile {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let tile_value = value % 10;
        let tile_type = value / 10;

        match tile_type {
            0 => Ok(Tile::Wan(tile_value)),
            1 => Ok(Tile::Suo(tile_value)),
            2 => Ok(Tile::Tong(tile_value)),
            3 => Ok(Tile::Feng(FengType::try_from(tile_value)?)),
            4 => Ok(Tile::Dragon(DragonType::try_from(tile_value)?)),
            5 => Ok(Tile::Hua(HuaType::try_from(tile_value)?)),
            6 => Ok(Tile::Animal(AnimalType::try_from(tile_value)?)),
            invalid_value => Err(format!("Cannot convert {} into Tile", invalid_value)),
        }
    }
}

impl From<Tile> for i8 {
    fn from(val: Tile) -> Self {
        match val {
            Tile::Wan(v) => v,
            Tile::Suo(v) => 10 + v,
            Tile::Tong(v) => 20 + v,
            Tile::Feng(v) => 30 + v as i8,
            Tile::Dragon(v) => 40 + v as i8,
            Tile::Hua(v) => 50 + v as i8,
            Tile::Animal(v) => 60 + v as i8,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum FengType {
    East = 1,
    South = 2,
    West = 3,
    North = 4,
}

impl TryFrom<i8> for FengType {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FengType::East),
            2 => Ok(FengType::South),
            3 => Ok(FengType::West),
            4 => Ok(FengType::North),
            invalid_value => Err(format!("Cannot convert {} into FengType", invalid_value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum DragonType {
    Zhong = 1,
    Baiban = 2,
    Fa = 3,
}

impl TryFrom<i8> for DragonType {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DragonType::Fa),
            2 => Ok(DragonType::Zhong),
            3 => Ok(DragonType::Baiban),
            invalid_value => Err(format!("Cannot convert {} into DragonType", invalid_value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum HuaType {
    RedOne = 1,
    RedTwo = 2,
    RedThree = 3,
    RedFour = 4,
    BlueOne = 5,
    BlueTwo = 6,
    BlueThree = 7,
    BlueFour = 8,
}

impl TryFrom<i8> for HuaType {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HuaType::RedOne),
            2 => Ok(HuaType::RedTwo),
            3 => Ok(HuaType::RedThree),
            4 => Ok(HuaType::RedFour),
            5 => Ok(HuaType::BlueOne),
            6 => Ok(HuaType::BlueTwo),
            7 => Ok(HuaType::BlueThree),
            8 => Ok(HuaType::BlueFour),
            invalid_value => Err(format!("Cannot convert {} into HuaType", invalid_value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AnimalType {
    Cat = 1,
    Rat = 2,
    Rooster = 3,
    Centipede = 4,
}

impl TryFrom<i8> for AnimalType {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AnimalType::Cat),
            2 => Ok(AnimalType::Rat),
            3 => Ok(AnimalType::Rooster),
            4 => Ok(AnimalType::Centipede),
            invalid_value => Err(format!("Cannot convert {} into AnimalType", invalid_value)),
        }
    }
}

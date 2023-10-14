#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Wan(i8),
    Suo(i8),
    Tong(i8),
    Feng(Option<FengType>),
    Dragon(Option<DragonType>),
    Hua(Option<HuaType>),
    Animal(Option<AnimalType>),
    Invalid()
}

impl From<i8> for Tile {
    fn from(value: i8) -> Self {
        let tile_value = value % 10;
        let tile_type = value / 10;

        match tile_type {
            0 => Tile::Wan(tile_value),
            1 => Tile::Suo(tile_value),
            2 => Tile::Tong(tile_value),
            3 => Tile::Feng(match FengType::try_from(tile_value) {
                Ok(feng_type) => Some(feng_type),
                Err(e) => {
                    println!("Err: {}", e);
                    None
                }
            }),
            4 => Tile::Dragon(match DragonType::try_from(tile_value) {
                Ok(dragon_type) => Some(dragon_type),
                Err(e) => {
                    println!("Err: {}", e);
                    None
                }
            }),
            5 => Tile::Hua(match HuaType::try_from(tile_value) {
                Ok(hua_type) => Some(hua_type),
                Err(e) => {
                    println!("Err: {}", e);
                    None
                }
            }),
            6 => Tile::Animal(match AnimalType::try_from(tile_value) {
                Ok(animal_type) => Some(animal_type),
                Err(e) => {
                    println!("Err: {}", e);
                    None
                }
            }),
            _ => Tile::Invalid()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub enum Meld {
    Chi(Tile, Tile, Tile),
    Pong(Tile),
    Gang(Tile),
    AnGang(Tile),
}

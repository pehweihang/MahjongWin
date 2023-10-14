#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Wan(i8),
    Suo(i8),
    Tong(i8),
    Feng(FengType),
    Dragon(DragonType),
    Hua(u8, HuaColor),
    Animal(AnimalType),
}

impl From<i8> for Tile {
    fn from(value: i8) -> Self {
        let v = value % 10; 
        let col = value/10;
        match col {
            0 => Tile::Wan(v),
            1 => Tile::Suo(v),
            2 => Tile::Tong(v),
            3 => Tile::Feng(v),
            4 => Tile::Dragon(v),
            5 => Tile::Feng(v),
            
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
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FengType::East),
            2 => Ok(FengType::South),
            3 => Ok(FengType::West),
            4 => Ok(FengType::North),
            x => Err(format!("Cannot convert {} into FengType", x))
            
        }
    }
}



#[derive(Debug, PartialEq, Eq, Hash)] 
pub enum DragonType {
    Zhong = 1,
    Baiban = 2,
    Fa = 3
}

#[derive(Debug, PartialEq, Eq, Hash)] 
pub enum HuaColor {
    Red = 1,
    Blue = 2,
}

#[derive(Debug, PartialEq, Eq, Hash)] 
pub enum AnimalType {
    Cat = 1,
    Rat = 2,
    Rooster = 3,
    Centipede = 4,
}

#[derive(Debug)]
pub enum Meld {
    Chi(Tile, Tile, Tile),
    Pong(Tile),
    Gang(Tile),
    AnGang(Tile),
}

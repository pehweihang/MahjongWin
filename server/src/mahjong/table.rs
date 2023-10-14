use super::{player::Player, tile::{Tile, DragonType, FengType, HuaType, AnimalType}};

#[derive(Debug, Default)]
struct Table {
    players: [Player; 4],

    current_feng: Feng,
    banker: usize,

    tiles: Vec<Tile>,
    discards: Vec<Tile>,
    next_draw: usize
}

impl Table {
    pub fn new(players: [Player; 4]) -> Self {
        let mut tiles = vec![];
        // 4 copies of each tile
        for _ in 0..4 {
            // number tiles
            for i in 1..10 {
                tiles.push(Tile::Wan(i));
                tiles.push(Tile::Suo(i));
                tiles.push(Tile::Tong(i));
            } 
            tiles.push(Tile::Dragon(DragonType::Fa));
            tiles.push(Tile::Dragon(DragonType::Zhong));
            tiles.push(Tile::Dragon(DragonType::Baiban));

            tiles.push(Tile::Feng(FengType::East));
            tiles.push(Tile::Feng(FengType::North));
            tiles.push(Tile::Feng(FengType::West));
            tiles.push(Tile::Feng(FengType::South));
        };

        tiles.push(Tile::Hua(HuaType::RedOne));
        tiles.push(Tile::Hua(HuaType::RedTwo));
        tiles.push(Tile::Hua(HuaType::RedThree));
        tiles.push(Tile::Hua(HuaType::RedFour));
        tiles.push(Tile::Hua(HuaType::BlueOne));
        tiles.push(Tile::Hua(HuaType::BlueTwo));
        tiles.push(Tile::Hua(HuaType::BlueThree));
        tiles.push(Tile::Hua(HuaType::BlueFour));

        tiles.push(Tile::Animal(AnimalType::Cat));
        tiles.push(Tile::Animal(AnimalType::Rat));
        tiles.push(Tile::Animal(AnimalType::Rooster));
        tiles.push(Tile::Animal(AnimalType::Centipede));

        Self {tiles, players, ..Default::default() }

    }
}

#[derive(Debug, Default)]
enum Feng{
    #[default]
    East,
    South,
    West,
    North,
}

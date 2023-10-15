use rand::{seq::SliceRandom, Rng};

use super::{
    player::Player,
    tile::{AnimalType, DragonType, FengType, HuaType, Tile},
};

#[derive(Debug, Default)]
struct Table {
    players: [Player; 4],

    current_feng: Feng,
    banker: usize,

    tiles: Vec<Tile>,
    discards: Vec<Tile>,
    next_draw: usize,

    current_turn: usize,
}

impl Table {
    pub fn new(starting_chips: i32) -> Self {
        let players = [
            Player::new(starting_chips),
            Player::new(starting_chips),
            Player::new(starting_chips),
            Player::new(starting_chips),
        ];
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
        }

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

        Self {
            tiles,
            players,
            ..Default::default()
        }
    }

    pub fn draw_next_tile(&mut self, player_number: usize) {
        // TODO error handling
        self.players[player_number].draw(self.tiles.get(self.next_draw).unwrap().clone());
        self.next_draw += 1;
    }

    pub fn new_game(&mut self) {
        self.players.shuffle(&mut rand::thread_rng());
        // first player is the banker for the first round
        self.banker = 0;
        self.current_feng = Feng::East;
    }

    pub fn new_round(&mut self) {
        self.tiles.shuffle(&mut rand::thread_rng());
        self.next_draw = 0;

        for i in 0..4 {
            for _ in 0..12 {
                self.draw_next_tile(i);
            }
        }
        self.draw_next_tile(self.banker);
    }

    pub fn next_turn(&mut self) {
        // TODO
        // player draws
        // player action (zimuo, angang)
        // player discards
        // other player actions (hu, pong, gang, chi)
        // set next players turn
        todo!();
    }
}

#[derive(Debug, Default)]
enum Feng {
    #[default]
    East,
    South,
    West,
    North,
}

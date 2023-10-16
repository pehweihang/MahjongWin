use rand::seq::SliceRandom;
use thiserror::Error;

use super::{
    player::{Action, Player},
    tile::{AnimalType, DragonType, FengType, HuaType, Tile}, meld::AnGang,
};

#[derive(Debug, Default)]
struct Table {
    players: [Player; 4],

    current_feng: Feng,
    banker: usize,

    tiles: Vec<Tile>,
    discards: Vec<Tile>,
    next_draw: usize,

    current_player: usize,
    round_state: RoundState,
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

    fn draw_next_tile(&mut self, player_number: usize) {
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
        self.round_state = RoundState::CurrentPlayerDraw;
    }

    pub fn next_draw(&mut self) -> Result<Vec<PlayerAction>, GameLoopError> {
        if !matches!(self.round_state, RoundState::CurrentPlayerDraw) {
            return Err(GameLoopError::InvalidStateError);
        }

        self.draw_next_tile(self.current_player);
        let mut current_player_actions = vec![];
        let angangs = self.players[self.current_player].get_angang();
        for ag in angangs {
            current_player_actions.push(PlayerAction {
                player_index: self.current_player,
                action: Action::Meld(ag),
            })
        }

        if self.players[self.current_player].can_zimuo() {
            current_player_actions.push(PlayerAction {
                player_index: self.current_player,
                action: Action::ZiMuo,
            });
        }

        self.round_state = RoundState::CurrentPlayerAction;

        Ok(current_player_actions)
    }

    // TODO
    // player action (zimuo, angang)
    // player discards
    // other player actions (hu, pong, gang, chi)
    // set next players turn
    pub fn current_player_action(&mut self, player_action: &PlayerAction) -> Result<(), GameLoopError> {
        if !matches!(self.round_state, RoundState::CurrentPlayerAction) {
            return Err(GameLoopError::InvalidStateError);
        }
        match player_action.action {
            Action::ZiMuo => todo!(),
            Action::Meld(AnGang(t)) => {todo!()},
            Action::None => {},
            _ => {
                
            }
        }


        Ok(())
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

#[derive(Debug, Default)]
enum RoundState {
    #[default]
    CurrentPlayerDraw,
    CurrentPlayerAction,
    OtherPlayerAction,
    RoundEnd,
}

type PlayerIndex = usize;

#[derive(Debug)]
struct PlayerAction {
    player_index: usize,
    action: Action,
}

#[derive(Debug, Error)]
enum GameLoopError {
    #[error("Player does not have required tiles to perform action: {action:?}")]
    IllegalMoveError { action: String },
    #[error("Cannot perform action in this state.")]
    InvalidStateError,
}

use crate::board::Board;
use crate::player::Player;
use crate::tile::{Rotation, Tile, TileBox};

#[derive(Debug)]
pub struct TilePlacement {
    pub tile: Tile,
    pub row: usize,
    pub column: usize,
    pub rotation: Rotation,
}

#[derive(Debug)]
pub enum Move {
    CantPlay,
    Move(Vec<TilePlacement>),
}

#[derive(Debug)]
pub struct Nile {
    board: Board,
    tile_box: TileBox,
    players: Vec<Player>,
    current_turn: usize,
}

#[derive(Debug)]
pub enum GameState<'g> {
    Turn(&'g str),
    EndOfGame,
}

impl Nile {
    pub fn new(player_names: Vec<String>) -> Result<Self, String> {
        if player_names.len() < 2 || player_names.len() > 4 {
            Err("Nile is a game for 2-4 players".to_owned())
        } else {
            let mut tile_box = TileBox::new();
            let mut players = Vec::with_capacity(player_names.len());
            player_names.into_iter().for_each(|player| {
                players.push(Player::new(player, &mut tile_box));
            });
            Ok(Self {
                board: Board::new(),
                tile_box,
                players,
                current_turn: 0,
            })
        }
    }

    pub fn take_move(&mut self, m: Move) -> Result<(), String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");
        let res =  match m {
            Move::CantPlay => {
                let mut penalty = 0;
                for tile in player.discard_tiles() {
                    penalty -= tile.score();
                    self.tile_box.discard(tile);
                }
                player.add_score(penalty);
                Ok(())
            },
            Move::Move(placements) => {
                Ok(())
            }
        };
        player.end_turn(&mut self.tile_box);
        self.advance_turn();
        res
    }

    fn advance_turn(&mut self) {
        self.current_turn += 1;
        if self.current_turn >= self.players.len() {
            self.current_turn = 0;
        }
    }
}

use crate::board::Board;
use crate::player::Player;
use crate::tile::{self, TileBox};
use crate::event::{self, Event, Log};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Nile {
    board: Board,
    tile_box: TileBox,
    players: Vec<Player>,
    current_turn: usize,
    log: Log,
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
                log: Log::new(),
            })
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<(), String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");

        match event {
            Event::PlaceTile(event::TilePlacement{tile, coordinates, rotation}) => {
                // TODO: validate tile placement
                self.board.place_tile(coordinates, tile::TilePlacement{tile, rotation});
                // TODO: show theoretical score
            }
            Event::RotateTile(event::Rotation{coordinates, rotation}) => {
                if self.board.get_cell(coordinates).is_empty() {
                    return Err("Cell is empty".to_owned())
                }
                // TODO: tile at coordinates validate tile was placed on this turn
            }
            Event::RemoveTile(coordinates) => {
                self.board.remove_tile(coordinates);
            }
            Event::CantPlay => {
                self.advance_turn();
            }
            Event::EndTurn => {
                self.advance_turn();
            }
            Event::Undo | Event::Redo => ()
        };
        if let Some(event) = self.log.handle_event(event) {
            self.handle_event(event)
        } else {
            Ok(())
        }
    }

    // pub fn take_move(&mut self, m: Move) -> Result<(), String> {
    //     let player = self.players.get_mut(self.current_turn).expect("Player");
    //     let res = match m {
    //         Move::CantPlay => {
    //             let mut penalty = 0;
    //             for tile in player.discard_tiles() {
    //                 penalty -= tile.score();
    //                 self.tile_box.discard(tile);
    //             }
    //             player.add_score(penalty);
    //             Ok(())
    //         }
    //         Move::Move(placements) => Ok(()),
    //     };
    //     player.end_turn(&mut self.tile_box);
    //     self.advance_turn();
    //     res
    // }

    fn advance_turn(&mut self) {
        self.current_turn += 1;
        if self.current_turn >= self.players.len() {
            self.current_turn = 0;
        }
    }
}

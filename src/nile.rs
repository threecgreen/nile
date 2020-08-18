use crate::board::Board;
use crate::event::{self, Event, Log};
use crate::player::Player;
use crate::score::{sum_scores, TurnScore};
use crate::tile::{self, TileBox};

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
            let player_count = players.len();
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

        // TODO: separate `match` into private function
        match event {
            Event::PlaceTile(event::TilePlacement {
                tile,
                coordinates,
                rotation,
            }) => {
                player
                    .place_tile(tile)
                    .ok_or_else(|| format!("Player doesn't have a {:?}", tile))?;
                let event_score = self
                    .board
                    .place_tile(coordinates, tile::TilePlacement { tile, rotation })?;
                let turn_score = player.add_score(event_score);
                // TODO: show theoretical score
            }
            Event::RotateTile(event::Rotation {
                coordinates,
                rotation,
            }) => {
                // TODO: tile at coordinates validate tile was placed on this turn
                self.board.rotate_tile(coordinates, rotation)?;
            }
            Event::RemoveTile(coordinates) => {
                if let Some((tile_placement, event_score)) = self.board.remove_tile(coordinates) {
                    player.return_tile(tile_placement.tile);
                    let turn_score = player.add_score(event_score);
                } else {
                    return Err("No tile there".to_owned());
                }
            }
            Event::CantPlay => {
                if self.log.can_undo() {
                    return Err("Player has made moves this turn".to_owned());
                }
                // FIXME: can pass in `self.tile_box` to `player` and have it handle most of this
                let tiles = player.discard_tiles();
                let tile_score = tiles.iter().fold(0, |acc, t| acc + t.score());
                // Add negative score
                player.add_score(TurnScore::new(0, tile_score));
                self.tile_box.discard(tiles);
                self.advance_turn();
            }
            Event::EndTurn => {
                player.end_turn(&mut self.tile_box);
                self.advance_turn();
            }
            Event::Undo | Event::Redo => (),
        };
        // `self.log.handle_event` returns `Some` for `Event::Redo` and `Event::Undo`
        if let Some(event) = self.log.handle_event(event) {
            // Recurse
            self.handle_event(event)
        } else {
            Ok(())
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }

    pub fn current_turn(&self) -> usize {
        self.current_turn
    }
}

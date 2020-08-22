use crate::board::{Board, TilePlacement};
use crate::log::{Event, Log};
use crate::path::{TilePath, TilePathType};
use crate::player::Player;
use crate::score::TurnScore;
use crate::tile::{self, Coordinates, Rotation, Tile, TileBox};

use js_sys::Array;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

/// Holds all state for one game
#[wasm_bindgen]
#[derive(Debug)]
pub struct Nile {
    // the game board
    board: Board,
    /// tiles that have not yet been drawn
    tile_box: TileBox,
    /// player-specific data
    players: Vec<Player>,
    /// the index in `players` of the player whose turn it is
    current_turn: usize,
    /// event log for undoing and redoing
    log: Log,
}

impl Nile {
    pub fn new(player_names: Vec<String>) -> Result<Self, String> {
        if player_names.len() < 2 || player_names.len() > 4 {
            Err("Nile is a game for 2-4 players".to_owned())
        } else {
            let mut tile_box = TileBox::new();
            let players = player_names
                .into_iter()
                .map(|player| Player::new(player, &mut tile_box))
                .collect();
            Ok(Self {
                board: Board::new(),
                tile_box,
                players,
                current_turn: 0,
                log: Log::new(),
            })
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn current_turn(&self) -> usize {
        self.current_turn
    }

    pub fn can_undo(&self) -> bool {
        self.log.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.log.can_redo()
    }

    pub fn place_tile(
        &mut self,
        tile_path_type: TilePathType,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<TurnScore, String> {
        let tile = Tile::from(&tile_path_type);
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player
            .place_tile(tile)
            .ok_or_else(|| format!("Player doesn't have a {:?}", tile))?;
        let event_score = self
            .board
            .place_tile(
                coordinates,
                TilePlacement::new(tile_path_type.clone(), rotation),
            )
            .map_err(|e| {
                // Player's tile rack should be unchanged
                player.return_tile(tile);
                e
            })?;
        let turn_score = player.add_score(event_score);
        self.log.place_tile(tile_path_type, coordinates, rotation);
        Ok(turn_score)
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: tile::Coordinates,
        rotation: Rotation,
    ) -> Result<(), String> {
        let old_rotation = self
            .board
            .get_cell(coordinates.0, coordinates.1)
            .tile()
            .ok_or_else(|| "No tile there".to_owned())?
            .rotation();
        if !self.log.cell_changed_in_turn(coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        self.board.rotate_tile(coordinates, rotation)?;
        self.log.rotate_tile(coordinates, old_rotation, rotation);
        Ok(())
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Result<TurnScore, String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");
        if !self.log.cell_changed_in_turn(coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        let (tile_placement, event_score) = self
            .board
            .remove_tile(coordinates)
            .ok_or_else(|| "No tile there".to_owned())?;
        player.return_tile(Tile::from(tile_placement.tile_path_type()));
        let turn_score = player.add_score(event_score);
        self.log.remove_tile(
            tile_placement.tile_path_type().clone(),
            coordinates,
            tile_placement.rotation(),
        );
        Ok(turn_score)
    }

    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        tile_path: TilePath,
    ) -> Result<(), String> {
        if !self.log.cell_changed_in_turn(coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        let old_tile_path = self.board.update_universal_path(coordinates, tile_path)?;
        self.log
            .update_universal_path(coordinates, old_tile_path, tile_path);
        Ok(())
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> Result<TurnScore, String> {
        if !self.log.cell_changed_in_turn(old_coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        // FIXME: implement
        self.log.move_tile(old_coordinates, new_coordinates);
        // TODO: return score diff
        Ok(TurnScore::default())
    }

    pub fn cant_play(&mut self) -> Result<(), String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");
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
        self.log.cant_play();
        // TODO: return turn score and tiles
        Ok(())
    }

    pub fn end_turn(&mut self) -> Result<EndTurnUpdate, String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");
        let turn_score = player.end_turn(&mut self.tile_box);
        let tiles = player.tiles().to_owned();
        self.advance_turn();
        self.log.end_turn();
        Ok(EndTurnUpdate { tiles, turn_score })
    }

    pub fn undo(&mut self) -> Result<Option<TurnScore>, String> {
        let event = self
            .log
            .begin_undo()
            .ok_or_else(|| "Nothing to undo".to_owned())?;
        let res = self.dispatch(event);
        self.log.end_undo();
        res
    }

    pub fn redo(&mut self) -> Result<Option<TurnScore>, String> {
        let event = self
            .log
            .redo()
            .ok_or_else(|| "Nothing to redo".to_owned())?;
        self.dispatch(event)
    }

    fn dispatch(&mut self, event: Event) -> Result<Option<TurnScore>, String> {
        match event {
            Event::PlaceTile(tpe) => self
                .place_tile(tpe.tile_path_type, tpe.coordinates, tpe.rotation)
                .map(Some),
            Event::RotateTile(re) => self
                .rotate_tile(re.new.coordinates, re.new.rotation)
                .map(|_| None),
            Event::RemoveTile(tpe) => self.remove_tile(tpe.coordinates).map(Some),
            Event::UpdateUniversalPath(uup) => self
                .update_universal_path(uup.coordinates, uup.new_tile_path)
                .map(|_| None),
            Event::MoveTile(mte) => self.move_tile(mte.old, mte.new).map(Some),
            e => Err(format!("Invalid event: {:?}", e)),
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EndTurnUpdate {
    turn_score: TurnScore,
    tiles: VecDeque<Tile>,
}

#[wasm_bindgen]
impl EndTurnUpdate {
    pub fn get_turn_score(&self) -> TurnScore {
        self.turn_score
    }

    pub fn get_tiles(&self) -> Array {
        self.tiles
            .clone()
            .into_iter()
            .map(|t| JsValue::from_serde(&t).unwrap())
            .collect()
    }
}

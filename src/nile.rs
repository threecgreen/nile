use crate::board::Board;
use crate::log::{Event, Log};
use crate::player::Player;
use crate::score::TurnScore;
use crate::tile::{self, Coordinates, Rotation, Tile, TileBox};

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
        tile: Tile,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<TurnScore, String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player
            .place_tile(tile)
            .ok_or_else(|| format!("Player doesn't have a {:?}", tile))?;
        let event_score = self
            .board
            .place_tile(coordinates, tile::TilePlacement { tile, rotation })
            .map_err(|e| {
                // Player's tile rack should be unchanged
                player.return_tile(tile);
                e
            })?;
        let turn_score = player.add_score(event_score);
        self.log.place_tile(tile, coordinates, rotation);
        // TODO: return score diff
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
            .rotation;
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
        player.return_tile(tile_placement.tile);
        let turn_score = player.add_score(event_score);
        self.log
            .remove_tile(tile_placement.tile, coordinates, tile_placement.rotation);
        Ok(turn_score)
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

    pub fn end_turn(&mut self) -> Result<i16, String> {
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player.end_turn(&mut self.tile_box);
        self.advance_turn();
        self.log.end_turn();
        // TODO: return turn score and tiles
        Ok(0)
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
                .place_tile(tpe.tile, tpe.coordinates, tpe.rotation)
                .map(Some),
            Event::RotateTile(re) => self
                .rotate_tile(re.new.coordinates, re.new.rotation)
                .map(|_| None),
            Event::RemoveTile(tpe) => self.remove_tile(tpe.coordinates).map(Some),
            Event::MoveTile(mte) => self.move_tile(mte.old, mte.new).map(Some),
            e => Err(format!("Invalid event: {:?}", e)),
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }
}

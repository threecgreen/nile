use crate::ai::{Brute, CPUPlayer};
use crate::board::{Board, TilePlacement};
use crate::log;
use crate::log::{Event, Log, TilePlacementEvent};
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
    /// Count of consecutive "can't plays"
    cant_play_count: u8,
    /// Whether the game has ended
    has_ended: bool,
}

impl Nile {
    pub fn new(player_names: Vec<String>, ai_count: usize) -> Result<Self, String> {
        if player_names.len() + ai_count < 2 || player_names.len() + ai_count > 4 {
            Err("Nile is a game for 2-4 players".to_owned())
        } else {
            let mut tile_box = TileBox::new();
            let mut players: Vec<Player> = player_names
                .into_iter()
                .map(|player| Player::new(player, &mut tile_box, false))
                .collect();
            for i in 0..ai_count {
                players.push(Player::new(format!("cpu{}", i), &mut tile_box, true))
            }
            Ok(Self {
                board: Board::new(),
                tile_box,
                players,
                current_turn: 0,
                log: Log::new(),
                cant_play_count: 0,
                has_ended: false,
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

    pub fn has_ended(&self) -> bool {
        self.has_ended
    }

    pub fn place_tile(
        &mut self,
        tile_path_type: TilePathType,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<TurnScore, String> {
        self.if_not_ended()?;
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
        self.if_not_ended()?;
        let old_rotation = self
            .board
            .cell(coordinates)
            .ok_or_else(|| "Invalid coordinates".to_owned())?
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
        self.if_not_ended()?;
        if !self.log.cell_changed_in_turn(coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        let (tile_placement, event_score) = self
            .board
            .remove_tile(coordinates)
            .ok_or_else(|| "No tile there".to_owned())?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
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
        self.if_not_ended()?;
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
        self.if_not_ended()?;
        if !self.log.cell_changed_in_turn(old_coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        let score_change = self.board.move_tile(old_coordinates, new_coordinates)?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        let turn_score = player.add_score(score_change);
        self.log.move_tile(old_coordinates, new_coordinates);
        Ok(turn_score)
    }

    pub fn cant_play(&mut self) -> Result<EndTurnUpdate, String> {
        self.if_not_ended()?;
        let player_count = self.players.len();
        let player = self.players.get_mut(self.current_turn).expect("Player");
        if self.log.can_undo() {
            return Err("Player has made moves this turn".to_owned());
        }
        // FIXME: can pass in `self.tile_box` to `player` and have it handle most of this
        let tiles = player.discard_tiles();
        let tile_score = tiles.iter().fold(0, |acc, t| acc + t.score());
        let turn_score = TurnScore {
            add: 0,
            sub: tile_score,
        };
        player.add_score(turn_score);
        self.tile_box.discard(tiles);
        player.end_turn(&mut self.tile_box);
        self.cant_play_count += 1;
        self.has_ended = self.cant_play_count as usize == player_count;
        let update = EndTurnUpdate {
            tiles: player.tiles().clone(),
            turn_score,
            game_has_ended: self.has_ended,
        };
        self.advance_turn();
        self.log.cant_play();
        Ok(update)
    }

    pub fn end_turn(&mut self) -> Result<EndTurnUpdate, String> {
        self.if_not_ended()?;
        self.has_ended = self
            .board
            .validate_turns_moves(self.log.current_turn_coordinates())?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        let turn_score = player.end_turn(&mut self.tile_box);
        let tiles = player.tiles().to_owned();
        self.advance_turn();
        self.log.end_turn();
        // Reset count
        self.cant_play_count = 0;

        Ok(EndTurnUpdate {
            tiles,
            turn_score,
            game_has_ended: self.has_ended,
        })
    }

    pub fn undo(&mut self) -> Result<Option<TurnScore>, String> {
        self.if_not_ended()?;
        let event = self
            .log
            .begin_undo()
            .ok_or_else(|| "Nothing to undo".to_owned())?;
        let res = self.dispatch(event);
        self.log.end_undo();
        res
    }

    pub fn redo(&mut self) -> Result<Option<TurnScore>, String> {
        self.if_not_ended()?;
        let event = self
            .log
            .redo()
            .ok_or_else(|| "Nothing to redo".to_owned())?;
        self.dispatch(event)
    }

    /// Process a CPU turn
    pub fn take_cpu_turn(&mut self) -> Option<CPUTurnUpdate> {
        if self.has_ended {
            return None;
        }
        let player = self.players.get(self.current_turn).expect("player");
        if !player.is_cpu() {
            return None;
        }
        let player_id = self.current_turn;
        // Hard-code CPU implementation for now
        Some(
            match Brute::new(self.players.len()).take_turn(
                player.tiles(),
                &self.board,
                player.total_score(),
                self.other_player_scores(),
            ) {
                Some(tile_placement_events) => {
                    for tpe in tile_placement_events.iter() {
                        if let Err(err) = self.place_tile(
                            tpe.tile_path_type.clone(),
                            tpe.coordinates,
                            tpe.rotation,
                        ) {
                            log(&format!(
                                "Failed to place a tile from CPU player: {:?}",
                                err
                            ));
                            self.undo_all();
                            let end_turn_update = self.cant_play().unwrap();
                            return Some(CPUTurnUpdate {
                                placements: Vec::new(),
                                player_id,
                                turn_score: end_turn_update.turn_score,
                                game_has_ended: end_turn_update.game_has_ended,
                            });
                        }
                    }
                    match self.end_turn() {
                        Ok(end_turn_update) => CPUTurnUpdate {
                            placements: tile_placement_events,
                            player_id,
                            turn_score: end_turn_update.turn_score,
                            game_has_ended: end_turn_update.game_has_ended,
                        },
                        Err(e) => {
                            log(&format!("Failed to end CPU player turn: {:?}", e));
                            self.undo_all();
                            let end_turn_update = self.cant_play().unwrap();
                            CPUTurnUpdate {
                                placements: Vec::new(),
                                player_id,
                                turn_score: end_turn_update.turn_score,
                                game_has_ended: end_turn_update.game_has_ended,
                            }
                        }
                    }
                }
                None => {
                    let end_turn_update = self.cant_play().unwrap();
                    CPUTurnUpdate {
                        placements: Vec::new(),
                        player_id,
                        turn_score: end_turn_update.turn_score,
                        game_has_ended: end_turn_update.game_has_ended,
                    }
                }
            },
        )
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
            Event::CantPlay | Event::EndTurn => Err(format!("Unsupported event type: {:?}", event)),
        }
    }

    fn undo_all(&mut self) {
        while self.can_undo() {
            self.undo().expect("Undo event");
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }

    fn if_not_ended(&self) -> Result<(), String> {
        if self.has_ended {
            Err("Game has already ended".to_owned())
        } else {
            Ok(())
        }
    }

    /// Get scores of players other than the current player
    fn other_player_scores(&self) -> Vec<i16> {
        self.players
            .iter()
            .enumerate()
            .filter_map(|(id, player)| {
                if id == self.current_turn {
                    None
                } else {
                    Some(player.total_score())
                }
            })
            .collect()
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EndTurnUpdate {
    pub turn_score: TurnScore,
    tiles: VecDeque<Tile>,
    pub game_has_ended: bool,
}

#[wasm_bindgen]
impl EndTurnUpdate {
    pub fn get_tiles(&self) -> Array {
        self.tiles
            .clone()
            .into_iter()
            .map(|t| JsValue::from_serde(&t).unwrap())
            .collect()
    }
}

// TODO: consolidate into `EndTurnUpdate`
#[wasm_bindgen]
#[derive(Debug)]
pub struct CPUTurnUpdate {
    pub player_id: usize,
    pub turn_score: TurnScore,
    placements: Vec<TilePlacementEvent>,
    pub game_has_ended: bool,
}

pub mod wasm {
    use super::*;

    use crate::log;

    #[wasm_bindgen]
    impl CPUTurnUpdate {
        pub fn get_placements(&self) -> Array {
            self.placements
                .iter()
                .map(|tpe| JsValue::from(log::wasm::TilePlacementEvent::from(tpe.to_owned())))
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Nile {
        Nile::new(vec!["player1".to_owned(), "player2".to_owned()], 0).unwrap()
    }

    fn get_normal_tile(target: &mut Nile) -> Tile {
        target.players[0]
            .tiles()
            .iter()
            .find(|t| **t != Tile::Universal)
            .expect("Non-universal tile (there's max four)")
            .clone()
    }

    /// Placing and removing a tile should have no net effect on the score
    #[test]
    fn place_remove_no_score_change() {
        let mut target = setup();
        let tile = get_normal_tile(&mut target);
        let inter_score = target
            .place_tile(
                crate::path::wasm::TilePathType::tile_into_normal(tile)
                    .unwrap()
                    .into(),
                Coordinates::new(10, 0),
                Rotation::Clockwise90,
            )
            .unwrap();
        assert_ne!(inter_score, TurnScore::default());
        assert_ne!(inter_score.score(), 0);
        let final_score = target.remove_tile(Coordinates::new(10, 0)).unwrap();
        assert_eq!(final_score, TurnScore::default());
        assert_eq!(final_score.score(), 0);
    }

    #[test]
    fn move_tile_has_no_score_change_except_for_bonues() {
        let mut target = setup();
        let tile = get_normal_tile(&mut target);
        let begin_score = target
            .place_tile(
                crate::path::wasm::TilePathType::tile_into_normal(tile)
                    .unwrap()
                    .into(),
                Coordinates::new(10, 0),
                Rotation::Clockwise90,
            )
            .unwrap();
        let end_score = target
            // Neither cell has a bonus
            .move_tile(Coordinates::new(10, 0), Coordinates::new(9, 0))
            .unwrap();
        assert_eq!(begin_score, end_score);
    }
}

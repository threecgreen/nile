use std::collections::HashSet;

use crate::ai::{Brute, CPUPlayer};
use crate::board::{Board, TilePlacement};
use crate::log;
use crate::log::{Event, Log, TilePlacementEvent};
use crate::path::{TilePath, TilePathType};
use crate::player::{Player, TileArray};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation, Tile, TileBox};

use js_sys::Array;
use wasm_bindgen::prelude::*;

// FIXME: distinguish between:
//  * (strictly) game logic and state, i.e. nothing about UI
//      * current_turn_placements
//      * what gets invoked by the CPU players' moves invoke
//  * "executor" and busines logic:
//      * undo/redo
//      * selected_tile
//      * what invokes or executes the CPU players' moves
/// Holds all game state
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Nile {
    // the game board
    board: Board,
    /// tiles that have not yet been drawn
    tile_box: TileBox,
    /// player-specific data
    players: Vec<Player>,
    /// the index in `players` of the player whose turn it is
    current_turn: usize,
    /// whether the player has selected a tile in the UI. Many actions are performed on the
    /// selected tile
    selected_tile: Option<SelectedTile>,
    /// coordinates where tiles were place in the current turn. Could be derived from `log` but
    /// keeping it here is faster
    current_turn_placements: HashSet<Coordinates>,
    /// event log for undoing and redoing
    log: Log,
    /// Count of consecutive "can't plays"
    cant_play_count: u8,
    /// Whether the game has ended
    has_ended: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectedTile {
    /// index of selected tile in rack
    Rack(u8),
    Board(Coordinates),
}

impl Nile {
    pub fn new(player_names: Vec<String>, cpu_player_count: u8) -> Result<Self, String> {
        let player_count = player_names.len() + cpu_player_count as usize;
        if player_count < 2 || player_count > 4 {
            Err("Nile is a game for 2-4 players".to_owned())
        } else {
            let mut tile_box = TileBox::default();
            let mut players: Vec<Player> = player_names
                .into_iter()
                .map(|player| Player::new(player, &mut tile_box, false))
                .collect();
            for i in 1..=cpu_player_count {
                players.push(Player::new(format!("cpu{}", i), &mut tile_box, true))
            }
            Ok(Self {
                board: Board::new(),
                tile_box,
                players,
                current_turn: 0,
                selected_tile: None,
                current_turn_placements: HashSet::default(),
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

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_turn]
    }

    /// Returns the current `SelectedTile` if it exists, whether on the board or the rack
    pub fn selected_tile(&self) -> &Option<SelectedTile> {
        &self.selected_tile
    }

    /// If a board tile is currently selected, returns its coordinates
    pub fn selected_board_tile(&self) -> Option<Coordinates> {
        match self.selected_tile {
            Some(SelectedTile::Board(coordinates)) => Some(coordinates),
            _ => None,
        }
    }

    /// If a rack tile is currently selected, returns its index in the `current_player()`'s
    /// `.tiles()`
    pub fn selected_rack_tile(&self) -> Option<u8> {
        match self.selected_tile {
            Some(SelectedTile::Rack(rack_idx)) => Some(rack_idx),
            _ => None,
        }
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

    pub fn select_rack_tile(&mut self, rack_idx: u8) -> Result<(), String> {
        if self
            .current_player()
            .tiles()
            .get(rack_idx as usize)
            .is_some()
        {
            self.selected_tile = Some(SelectedTile::Rack(rack_idx));
            // no `log` call because selections are not undoable events
            Ok(())
        } else {
            Err(format!("Invalid rack index: {}", rack_idx))
        }
    }

    pub fn select_board_tile(&mut self, coordinates: Coordinates) -> Result<(), String> {
        if !self.current_turn_placements.contains(&coordinates) {
            return Err("Can only select tiles from this turn".to_owned());
        }
        self.board
            .cell(coordinates)
            .map_or(Err(format!("Invalid {:?}", coordinates)), |cell| {
                if cell.tile().is_some() {
                    self.selected_tile = Some(SelectedTile::Board(coordinates));
                    Ok(())
                } else {
                    Err(format!("No tile at {:?}", coordinates))
                }
            })
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
        self.selected_tile = Some(SelectedTile::Board(coordinates));
        self.current_turn_placements.insert(coordinates);
        self.log.place_tile(tile_path_type, coordinates, rotation);
        Ok(turn_score)
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
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

        if !self.current_turn_placements.contains(&coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        self.board.rotate_tile(coordinates, rotation)?;
        self.log.rotate_tile(coordinates, old_rotation, rotation);
        Ok(())
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Result<TurnScore, String> {
        self.if_not_ended()?;
        if !self.current_turn_placements.contains(&coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        let (tile_placement, event_score) = self
            .board
            .remove_tile(coordinates)
            .ok_or_else(|| "No tile there".to_owned())?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player.return_tile(Tile::from(tile_placement.tile_path_type()));
        let turn_score = player.add_score(event_score);
        assert!(self.current_turn_placements.remove(&coordinates));
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
        if !self.current_turn_placements.contains(&coordinates) {
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
        if !self.current_turn_placements.contains(&old_coordinates) {
            return Err("Can't change tiles from another turn".to_owned());
        }
        let score_change = self.board.move_tile(old_coordinates, new_coordinates)?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        let turn_score = player.add_score(score_change);
        assert!(self.current_turn_placements.remove(&old_coordinates));
        assert!(self.current_turn_placements.insert(new_coordinates));
        self.log.move_tile(old_coordinates, new_coordinates);
        Ok(turn_score)
    }

    /// Called when the current player _claims_ they can't play any tiles. If successful, ends their
    /// turn
    pub fn cant_play(&mut self) -> Result<EndTurnUpdate, String> {
        self.if_not_ended()?;
        let player_count = self.players.len();
        let player = self.players.get_mut(self.current_turn).expect("Player");
        if self.log.can_undo() {
            return Err("Player has made moves this turn".to_owned());
        }
        // TODO: Check if any playable moves
        let turn_score = player.cant_play(&mut self.tile_box);

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

    /// Called when a human player ends their turn normally (they played at least one tile)
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
        self.current_turn_placements.clear();
        // Reset count
        self.cant_play_count = 0;

        Ok(EndTurnUpdate {
            tiles,
            turn_score,
            // Also end game if there are no more tiles
            game_has_ended: self.has_ended,
        })
    }

    /// Attempt to undo an action
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

    /// Attempt to redo a previously-undone action
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
        let lists_of_moves = Brute::new(self.players.len()).take_turn(
            player.tiles(),
            &self.board,
            player.total_score(),
            self.other_player_scores(),
        );
        'list_of_moves: for tile_placement_events in lists_of_moves {
            for tpe in tile_placement_events.iter() {
                if let Err(err) =
                    self.place_tile(tpe.tile_path_type.clone(), tpe.coordinates, tpe.rotation)
                {
                    log(&format!(
                        "Failed to place a tile from CPU player: {:?}; TilePlacement: {:?}",
                        err, &tpe
                    ));
                    self.undo_all();
                    // continue outer for loop
                    continue 'list_of_moves;
                }
            }
            match self.end_turn() {
                Ok(end_turn_update) => {
                    return Some(CPUTurnUpdate {
                        placements: tile_placement_events,
                        player_id,
                        turn_score: end_turn_update.turn_score,
                        game_has_ended: end_turn_update.game_has_ended,
                        tile_count: self.tile_count(),
                    });
                }
                Err(e) => {
                    log(&format!(
                        "Failed to end CPU player turn: {:?}; Placements: {:?}",
                        e, tile_placement_events,
                    ));
                    self.undo_all();
                    continue;
                }
            }
        }
        // Either no moves to begin with or all returned moves were invalid
        let end_turn_update = self.cant_play().unwrap();
        Some(CPUTurnUpdate {
            placements: Vec::new(),
            player_id,
            turn_score: end_turn_update.turn_score,
            game_has_ended: end_turn_update.game_has_ended,
            tile_count: self.tile_count(),
        })
    }

    /// executes the given `event`. Used for undoing and redoing events at user request
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

    fn tile_count(&self) -> usize {
        self.players[self.current_turn].tiles().len()
    }

    fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
        self.has_ended = self.has_ended || self.players[self.current_turn].rack_is_empty();
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
    tiles: TileArray,
    pub game_has_ended: bool,
}

#[wasm_bindgen]
impl EndTurnUpdate {
    pub fn get_tiles(&self) -> Array {
        self.tiles
            .clone()
            .into_iter()
            .map(|t| JsValue::from_f64(t as i32 as f64))
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
    /// we don't need to display or expose what tiles the cpu actually has
    pub tile_count: usize,
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

    #[test]
    fn advance_turn_doesnt_unend_turn() {
        let mut target = setup();
        target.has_ended = true;
        assert_eq!(target.current_turn, 0);
        target.advance_turn();
        assert_eq!(target.current_turn, 1);
        assert!(target.has_ended);
    }
}

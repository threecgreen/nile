use std::collections::HashSet;
use std::rc::Rc;

use rand::Rng;

use crate::ai::{Brute, CPUPlayer};
use crate::board::{Board, TilePlacement};
use crate::error::{self, Error};
use crate::log::{Event, Log};
use crate::path::{TilePath, TilePathType};
use crate::player::Player;
use crate::tile::{Coordinates, Rotation, Tile, TileBox};

pub type ActionResult = Result<(), String>;

/// Handles high-level game and UI logic. Executes CPU players' moves, handles undo/redo
#[derive(Debug, Clone)]
pub struct Engine {
    nile: Rc<Nile>,
    /// whether the player has selected a tile in the UI. Many actions are performed on the
    /// selected tile
    selected_tile: Option<SelectedTile>,
    /// event log for undoing and redoing
    log: Log,
    /// coordinates of a cell on the board that caused an error
    error_cells: Option<HashSet<Coordinates>>,
}

impl Engine {
    pub fn new(player_names: Vec<String>, cpu_player_count: u8) -> Result<Self, String> {
        let nile = Nile::new(player_names, cpu_player_count)?;
        let mut engine = Self {
            nile: Rc::new(nile),
            selected_tile: None,
            log: Log::new(),
            error_cells: None,
        };
        // Necessary if the randomized first player is a cpu
        engine.take_cpu_turns_if_any();
        Ok(engine)
    }

    pub fn board(&self) -> &Board {
        self.nile.board()
    }

    pub fn rc_board(&self) -> &Rc<Board> {
        self.nile.rc_board()
    }

    pub fn players(&self) -> &Vec<Player> {
        self.nile.players()
    }

    pub fn current_turn(&self) -> usize {
        self.nile.current_turn()
    }

    pub fn current_player(&self) -> &Player {
        self.nile.current_player()
    }

    pub fn can_undo(&self) -> bool {
        self.log.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.log.can_redo()
    }

    pub fn current_turn_placements(&self) -> &HashSet<Coordinates> {
        self.nile.current_turn_placements()
    }

    pub fn has_ended(&self) -> bool {
        self.nile.has_ended()
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

    pub fn error_cells(&self) -> Option<&HashSet<Coordinates>> {
        self.error_cells.as_ref()
    }

    pub fn select_rack_tile(&mut self, rack_idx: u8) -> ActionResult {
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

    pub fn select_board_tile(&mut self, coordinates: Coordinates) -> ActionResult {
        if !self.nile.current_turn_placements.contains(&coordinates) {
            return Err("Can only select tiles from this turn".to_owned());
        }
        self.nile.board.cell(coordinates).map_or(
            Err(format!("Invalid {:?}", coordinates)),
            |cell| {
                if cell.tile().is_some() {
                    self.selected_tile = Some(SelectedTile::Board(coordinates));
                    Ok(())
                } else {
                    Err(format!("No tile at {:?}", coordinates))
                }
            },
        )
    }

    pub fn place_tile(&mut self, coordinates: Coordinates) -> ActionResult {
        match self.selected_tile {
            Some(SelectedTile::Rack(idx)) => {
                let tile = self
                    .current_player()
                    .tiles()
                    .get(idx as usize)
                    .ok_or_else(|| format!("Invalid selected tile index: {}", idx))?;
                let tile_path_type = TilePathType::from(*tile);
                let rotation = Rotation::default();
                let res = self
                    .mut_nile()
                    .place_tile(tile_path_type, coordinates, rotation);
                self.handle_error_cells(res)?;
                self.selected_tile = Some(SelectedTile::Board(coordinates));
                self.log.place_tile(tile_path_type, coordinates, rotation);
                Ok(())
            }
            Some(SelectedTile::Board(old_coordinates)) => {
                let res = self.mut_nile().move_tile(old_coordinates, coordinates);
                self.handle_error_cells(res)?;
                self.selected_tile = Some(SelectedTile::Board(coordinates));
                self.log.move_tile(old_coordinates, coordinates);
                Ok(())
            }
            // None => Err("No selected tile".to_owned()),
            None => Ok(()),
        }
    }

    pub fn rotate_selected_tile(&mut self, rotation: Rotation) -> ActionResult {
        let coordinates = self
            .selected_board_tile()
            .ok_or_else(|| "No selected board tile".to_owned())?;
        let old_rotation = self
            .nile
            .board()
            .cell(coordinates)
            .ok_or_else(|| "Invalid coordinates".to_owned())?
            .tile()
            .ok_or_else(|| "No tile there".to_owned())?
            .rotation();
        let res = self.mut_nile().rotate_tile(coordinates, rotation);
        self.handle_error_cells(res)?;
        self.log.rotate_tile(coordinates, old_rotation, rotation);
        Ok(())
    }

    pub fn remove_selected_tile(&mut self) -> ActionResult {
        let coordinates = self
            .selected_board_tile()
            .ok_or_else(|| "No selected board tile".to_owned())?;
        let res = self.mut_nile().remove_tile(coordinates);
        let old_tile_placement = self.handle_error_cells(res)?;
        // TODO: make removed tile in rack the new selected?
        self.selected_tile = None;
        self.log.remove_tile(
            old_tile_placement.tile_path_type().to_owned(),
            coordinates,
            old_tile_placement.rotation(),
        );
        Ok(())
    }

    pub fn update_selected_universal_path(&mut self, tile_path: TilePath) -> ActionResult {
        let coordinates = self
            .selected_board_tile()
            .ok_or_else(|| "No selected board tile".to_owned())?;
        let res = self
            .mut_nile()
            .update_universal_path(coordinates, tile_path);
        let old_tile_path = self.handle_error_cells(res)?;
        self.log
            .update_universal_path(coordinates, old_tile_path, tile_path);
        Ok(())
    }

    /// Attempt to undo an action
    pub fn undo(&mut self) -> ActionResult {
        self.handle_error_cells(self.nile.if_not_ended())?;
        let event = self
            .log
            .undo()
            .ok_or_else(|| "Nothing to undo".to_owned())?;
        self.dispatch(event)
    }

    /// Attempt to redo a previously-undone action
    pub fn redo(&mut self) -> ActionResult {
        self.handle_error_cells(self.nile.if_not_ended())?;
        let event = self
            .log
            .redo()
            .ok_or_else(|| "Nothing to redo".to_owned())?;
        self.dispatch(event)
    }

    pub fn end_turn(&mut self) -> Result<bool, String> {
        let res = self.mut_nile().end_turn();
        self.handle_error_cells(res)?;
        self.log.end_turn();
        self.selected_tile = None;
        self.error_cells = None;
        self.take_cpu_turns_if_any();
        Ok(self.has_ended())
    }

    pub fn cant_play(&mut self) -> Result<bool, String> {
        let res = self.mut_nile().cant_play();
        self.handle_error_cells(res)?;
        self.log.cant_play();
        self.selected_tile = None;
        self.error_cells = None;
        self.take_cpu_turns_if_any();
        Ok(self.has_ended())
    }

    fn take_cpu_turns_if_any(&mut self) {
        while !self.has_ended() && self.current_player().is_cpu() {
            if !self.take_cpu_turn() {
                break;
            }
        }
    }

    fn mut_nile(&mut self) -> &mut Nile {
        Rc::make_mut(&mut self.nile)
    }

    /// Sets error state after a call to `nile`
    fn handle_error_cells<T>(&mut self, res: error::Result<T>) -> Result<T, String> {
        match res {
            Ok(v) => {
                self.error_cells = None;
                Ok(v)
            }
            Err(Error::Msg(msg)) => {
                self.error_cells = None;
                Err(msg)
            }
            Err(Error::Cell(cell_error)) => {
                self.error_cells = Some(cell_error.coordinates);
                Err(cell_error.msg)
            }
        }
    }

    /// Process a CPU turn
    fn take_cpu_turn(&mut self) -> bool {
        if self.nile.has_ended {
            return false;
        }
        let player = self.current_player();
        if !player.is_cpu() {
            return false;
        }
        // Hard-code CPU implementation for now
        let lists_of_moves = Brute::new(self.nile.players.len()).take_turn(
            player.tiles(),
            &self.nile.board,
            player.total_score(),
            self.other_player_scores(),
        );
        let success = lists_of_moves.iter().any(|tile_placement_events| {
            let success = tile_placement_events.iter().all(|tpe| {
                if let Err(err) =
                    self.mut_nile()
                        .place_tile(tpe.tile_path_type, tpe.coordinates, tpe.rotation)
                {
                    crate::console::warn(&format!(
                        "Failed to place a tile from CPU player: {:?}; TilePlacement: {:?}",
                        err, &tpe
                    ));
                    self.undo_all();
                    return false;
                }
                // Add to log in case there's a problem with the moves and everything needs to be
                // undo
                self.log
                    .place_tile(tpe.tile_path_type, tpe.coordinates, tpe.rotation);
                true
            });
            if success {
                self.end_turn().map(|_| true).unwrap_or_else(|e| {
                    crate::console::warn(&format!(
                        "Failed to end CPU player turn: {:?}; Placements: {:?}",
                        e, tile_placement_events,
                    ));
                    self.undo_all();
                    false
                })
            } else {
                false
            }
        });
        if !success {
            // Either no moves to begin with or all returned moves were invalid
            let _end_turn_update = self.cant_play().unwrap();
        }
        success
    }

    fn undo_all(&mut self) {
        while self.can_undo() {
            self.undo().expect("Undo event");
        }
    }

    fn dispatch(&mut self, event: Event) -> ActionResult {
        match event {
            Event::PlaceTile(tpe) => {
                let res =
                    self.mut_nile()
                        .place_tile(tpe.tile_path_type, tpe.coordinates, tpe.rotation);
                self.handle_error_cells(res)?;
                self.selected_tile = Some(SelectedTile::Board(tpe.coordinates));
            }
            Event::RotateTile(re) => {
                self.selected_tile = Some(SelectedTile::Board(re.new.coordinates));
                let res = self
                    .mut_nile()
                    .rotate_tile(re.new.coordinates, re.new.rotation);
                self.handle_error_cells(res)?;
            }
            Event::RemoveTile(tpe) => {
                let res = self.mut_nile().remove_tile(tpe.coordinates);
                self.handle_error_cells(res)?;
                self.selected_tile = None;
            }
            Event::UpdateUniversalPath(uup) => {
                self.selected_tile = Some(SelectedTile::Board(uup.coordinates));
                let res = self
                    .mut_nile()
                    .update_universal_path(uup.coordinates, uup.new_tile_path);
                self.handle_error_cells(res)?;
            }
            Event::MoveTile(mte) => {
                let res = self.mut_nile().move_tile(mte.old, mte.new);
                self.handle_error_cells(res)?;
                self.selected_tile = Some(SelectedTile::Board(mte.new));
            }
            Event::CantPlay | Event::EndTurn => {
                return Err(format!("Unsupported event type: {:?}", event));
            }
        };
        Ok(())
    }

    /// Get scores of players other than the current player
    fn other_player_scores(&self) -> Vec<i16> {
        self.nile
            .players()
            .iter()
            .enumerate()
            .filter_map(|(id, player)| {
                if id == self.nile.current_turn() {
                    None
                } else {
                    Some(player.total_score())
                }
            })
            .collect()
    }
}

/// Holds all game state
#[derive(Debug, Clone)]
pub struct Nile {
    // the game board
    board: Rc<Board>,
    /// tiles that have not yet been drawn
    tile_box: TileBox,
    /// player-specific data
    players: Vec<Player>,
    /// the index in `players` of the player whose turn it is
    current_turn: usize,
    /// coordinates where tiles were place in the current turn. Could be derived from `log` in
    /// `Engine` but keeping it here is faster
    current_turn_placements: HashSet<Coordinates>,
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
        if !(2..=4).contains(&player_count) {
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
            let mut rng = rand::thread_rng();
            let first_turn = rng.gen_range(0, players.len());
            Ok(Self {
                board: Rc::new(Board::new()),
                tile_box,
                players,
                current_turn: first_turn,
                current_turn_placements: HashSet::default(),
                cant_play_count: 0,
                has_ended: false,
            })
        }
    }

    pub fn rc_board(&self) -> &Rc<Board> {
        &self.board
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    fn mut_board(&mut self) -> &mut Board {
        Rc::make_mut(&mut self.board)
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

    pub fn current_turn_placements(&self) -> &HashSet<Coordinates> {
        &self.current_turn_placements
    }

    pub fn has_ended(&self) -> bool {
        self.has_ended
    }

    pub fn place_tile(
        &mut self,
        tile_path_type: TilePathType,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> error::Result<()> {
        self.if_not_ended()?;
        let tile = Tile::from(&tile_path_type);
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player
            .place_tile(tile)
            .ok_or_else(|| Error::Msg(format!("Player doesn’t have a {:?}", tile)))?;
        let event_score = Rc::make_mut(&mut self.board)
            .place_tile(coordinates, TilePlacement::new(tile_path_type, rotation))
            .map_err(|e| {
                // Player's tile rack should be unchanged
                player.return_tile(tile);
                Error::Cell(e)
            })?;
        let _turn_score = player.add_score(event_score);
        self.current_turn_placements.insert(coordinates);
        Ok(())
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> error::Result<()> {
        self.if_not_ended()?;
        if !self.current_turn_placements.contains(&coordinates) {
            return Err(Error::cell(
                coordinates,
                "Can't change tiles from another turn".to_owned(),
            ));
        }
        self.mut_board()
            .rotate_tile(coordinates, rotation)
            .map_err(Error::Cell)?;
        Ok(())
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> error::Result<TilePlacement> {
        self.if_not_ended()?;
        if !self.current_turn_placements.contains(&coordinates) {
            return Err(Error::cell(
                coordinates,
                "Can’t change tiles from another turn".to_owned(),
            ));
        }
        let (tile_placement, event_score) = self
            .mut_board()
            .remove_tile(coordinates)
            .ok_or_else(|| Error::cell(coordinates, "No tile there".to_owned()))?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player.return_tile(Tile::from(tile_placement.tile_path_type()));
        player.add_score(event_score);
        assert!(self.current_turn_placements.remove(&coordinates));
        Ok(tile_placement)
    }

    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        tile_path: TilePath,
    ) -> error::Result<TilePath> {
        self.if_not_ended()?;
        if !self.current_turn_placements.contains(&coordinates) {
            return Err(Error::cell(
                coordinates,
                "Can’t change tiles from another turn".to_owned(),
            ));
        }
        let old_tile_path = self
            .mut_board()
            .update_universal_path(coordinates, tile_path)?;
        Ok(old_tile_path)
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> error::Result<()> {
        self.if_not_ended()?;
        if !self.current_turn_placements.contains(&old_coordinates) {
            return Err(Error::cell(
                old_coordinates,
                "Can’t change tiles from another turn".to_owned(),
            ));
        }
        let score_change = self
            .mut_board()
            .move_tile(old_coordinates, new_coordinates)
            .map_err(Error::Cell)?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        player.add_score(score_change);
        assert!(self.current_turn_placements.remove(&old_coordinates));
        assert!(self.current_turn_placements.insert(new_coordinates));
        Ok(())
    }

    /// Called when the current player _claims_ they can't play any tiles. If successful, ends their
    /// turn
    pub fn cant_play(&mut self) -> error::Result<bool> {
        self.if_not_ended()?;
        if !self.current_turn_placements.is_empty() {
            return Err(Error::Msg("Player has placed tiles this turn".to_owned()));
        }
        let player_count = self.players.len();
        let player = self.players.get_mut(self.current_turn).expect("Player");
        // TODO: Check if any playable moves
        let _turn_score = player.cant_play(&mut self.tile_box);

        self.cant_play_count += 1;
        self.has_ended = self.cant_play_count as usize == player_count;
        self.advance_turn();
        Ok(self.has_ended)
    }

    /// Called when a human player ends their turn normally (they played at least one tile)
    pub fn end_turn(&mut self) -> error::Result<bool> {
        self.if_not_ended()?;
        if self.current_turn_placements.is_empty() {
            return Err(Error::Msg("Can’t end turn normally without placing at least one tile. Use “can’t play” if there are no playable moves".to_owned()));
        }
        self.has_ended = Rc::make_mut(&mut self.board)
            .validate_turns_moves(self.current_turn_placements.clone())?;
        let player = self.players.get_mut(self.current_turn).expect("Player");
        let _turn_score = player.end_turn(&mut self.tile_box);
        self.advance_turn();
        // Reset count
        self.cant_play_count = 0;

        Ok(self.has_ended)
    }

    fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
        self.has_ended = self.has_ended || self.players[self.current_turn].rack_is_empty();
        self.current_turn_placements.clear();
    }

    fn if_not_ended(&self) -> error::Result<()> {
        if self.has_ended {
            Err(Error::Msg("Game has already ended".to_owned()))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Nile {
        let mut nile = Nile::new(vec!["player1".to_owned(), "player2".to_owned()], 0).unwrap();
        nile.current_turn = 0;
        nile
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
        assert_eq!(target.current_player().current_turn_score().score(), 0);
        let tile = get_normal_tile(&mut target);
        target
            .place_tile(
                TilePathType::from(tile),
                Coordinates(10, 0),
                Rotation::Clockwise90,
            )
            .unwrap();
        assert_ne!(target.current_player().current_turn_score().score(), 0);
        target.remove_tile(Coordinates(10, 0)).unwrap();
        assert_eq!(target.current_player().current_turn_score().score(), 0);
    }

    #[test]
    fn move_tile_has_no_score_change_except_for_bonues() {
        let mut target = setup();
        let tile = get_normal_tile(&mut target);
        let begin_score = target
            .place_tile(
                TilePathType::from(tile),
                Coordinates(10, 0),
                Rotation::Clockwise90,
            )
            .unwrap();
        let end_score = target
            // Neither cell has a bonus
            .move_tile(Coordinates(10, 0), Coordinates(9, 0))
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

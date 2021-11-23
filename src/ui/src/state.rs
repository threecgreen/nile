use nile::{Coordinates, Nile, TilePath, TilePathType, TilePlacement};
use std::collections::HashSet;
use yewdux::prelude::{Reducer, ReducerStore};

use crate::components::utils::update_if_changed;

#[derive(Clone)]
pub struct State {
    /// Main game state
    pub nile: nile::Nile,
    // FIXME: `nile`'s log already has this
    /// Used for determining if placed tile is movable, rotatable, etc.
    pub current_turn_tiles: HashSet<Coordinates>,
    // FIXME: move into `nile` to facilitate with undo and redo
    pub selected_tile: Option<SelectedTile>,
    pub modal: Option<Modal>,
}

pub enum Action {
    SelectRackTile(SelectRackTile),
    SelectBoardTile(Coordinates),
    /// place a tile on the board. It will be moved from its previous location
    PlaceTile(Coordinates),
    RotateSelectedTile(Rotation),
    RemoveSelectedTile,
    UpdateSelectedUniversalPath(TilePath),
    Undo,
    Redo,
    EndTurn,
    CantPlay,
    CpuTurn,
    SetError(String),
    SetEndOfGame(String),
    Dismiss,
    None,
}

#[derive(Clone, PartialEq)]
pub enum SelectedTile {
    Rack(SelectRackTile),
    Board(Coordinates),
}

#[derive(Clone, PartialEq)]
pub enum Modal {
    Error(String),
    EndOfGame(String),
}

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, PartialEq)]
pub struct SelectRackTile {
    pub rack_idx: u8,
}

pub type GameStore = ReducerStore<State>;

impl State {
    pub fn new_game(player_names: Vec<String>, cpu_player_count: u8) -> Self {
        Self {
            nile: Nile::new(player_names, cpu_player_count as usize).unwrap(),
            current_turn_tiles: HashSet::new(),
            selected_tile: None,
            modal: None,
        }
    }

    pub fn can_undo(&self) -> bool {
        self.nile.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.nile.can_redo()
    }

    pub fn has_selected_board_tile(&self) -> bool {
        match self.selected_tile {
            Some(SelectedTile::Board(_)) => true,
            _ => false,
        }
    }
}

impl Reducer for State {
    type Action = Action;

    fn new() -> Self {
        Self::new_game(vec!["default".to_owned()], 1)
    }

    fn reduce(&mut self, action: Self::Action) -> yewdux::prelude::Changed {
        match action {
            Action::SelectRackTile(select_rack_tile) => self.select_rack_tile(select_rack_tile),
            Action::SelectBoardTile(coordinates) => self.select_board_tile(coordinates),
            Action::PlaceTile(coordinates) => self.place_tile(coordinates),
            Action::RotateSelectedTile(rotation) => self.rotate_selected_tile(rotation),
            Action::RemoveSelectedTile => self.remove_selected_tile(),
            Action::UpdateSelectedUniversalPath(tile_path) => {
                self.update_selected_universal_path(tile_path)
            }
            Action::Undo => self.undo(),
            Action::Redo => self.redo(),
            Action::EndTurn => self.end_turn(),
            Action::CantPlay => self.cant_play(),
            Action::CpuTurn => self.cpu_turn(),
            Action::SetError(msg) => self.set_error(msg),
            Action::SetEndOfGame(msg) => self.set_end_of_game(msg),
            Action::Dismiss => self.dismiss(),
            Action::None => false,
        }
    }
}

impl State {
    fn select_rack_tile(
        &mut self,
        SelectRackTile { rack_idx }: SelectRackTile,
    ) -> yewdux::prelude::Changed {
        let current_player = self.nile.current_player();
        // validate
        if let Some(_tile) = current_player.tiles().get(rack_idx as usize) {
            // store
            self.selected_tile = Some(SelectedTile::Rack(SelectRackTile { rack_idx }));
            true
        } else {
            false
        }
    }

    fn select_board_tile(&mut self, coordinates: Coordinates) -> yewdux::prelude::Changed {
        // Only this turn's tile can be selected
        if self.current_turn_tiles.contains(&coordinates) {
            update_if_changed(
                &mut self.selected_tile,
                Some(SelectedTile::Board(coordinates)),
            )
        } else {
            false
        }
    }

    fn place_tile(&mut self, coordinates: Coordinates) -> yewdux::prelude::Changed {
        match self.selected_tile {
            Some(SelectedTile::Rack(SelectRackTile { rack_idx })) => {
                let current_player = self.nile.current_player();
                if let Some(tile) = current_player.tiles().get(rack_idx as usize) {
                    let tile_path_type = TilePathType::from(*tile);
                    match self
                        .nile
                        .place_tile(tile_path_type, coordinates, nile::Rotation::None)
                    {
                        Ok(_turn_score) => {
                            self.selected_tile = Some(SelectedTile::Board(coordinates));
                            true
                        }
                        Err(msg) => self.set_error(msg),
                    }
                } else {
                    false
                }
            }
            Some(SelectedTile::Board(old_coordinates)) => {
                match self.nile.move_tile(old_coordinates, coordinates) {
                    Ok(_turn_score) => {
                        self.selected_tile = Some(SelectedTile::Board(coordinates));
                        true
                    }
                    Err(msg) => self.set_error(msg),
                }
            }
            None => false,
        }
    }

    fn rotate_selected_tile(&mut self, rotation: Rotation) -> yewdux::prelude::Changed {
        match self.selected_tile {
            Some(SelectedTile::Board(coordinates)) => self
                .nile
                .board()
                .cell(coordinates)
                .and_then(|c| c.tile())
                .map_or(false, |tile_placement| {
                    let new_rotation = nile::ROTATIONS[(tile_placement.rotation() as i8 + {
                        if rotation == Rotation::Clockwise {
                            1
                        } else {
                            -1
                        }
                    }) as usize
                        % nile::ROTATIONS.len()];
                    self.nile
                        .rotate_tile(coordinates, new_rotation)
                        .map_or_else(|msg| self.set_error(msg), |()| true)
                }),
            _ => false,
        }
    }

    fn remove_selected_tile(&mut self) -> yewdux::prelude::Changed {
        match self.selected_tile {
            Some(SelectedTile::Board(coordinates)) => match self.nile.remove_tile(coordinates) {
                Ok(_turn_score) => {
                    self.selected_tile = Some(SelectedTile::Rack(SelectRackTile {
                        rack_idx: (self.nile.current_player().tiles().len() - 1) as u8,
                    }));
                    true
                }
                Err(msg) => self.set_error(msg),
            },
            _ => false,
        }
    }

    fn update_selected_universal_path(&mut self, tile_path: TilePath) -> yewdux::prelude::Changed {
        match self.selected_tile {
            Some(SelectedTile::Board(coordinates)) => self
                .nile
                .board()
                .cell(coordinates)
                .and_then(|c| c.tile())
                .map_or(false, |tile_placement| {
                    if let TilePathType::Universal(_old_tile_path) = tile_placement.tile_path_type()
                    {
                        self.nile
                            .update_universal_path(coordinates, tile_path)
                            .map(|()| true)
                            .unwrap_or_else(|msg| self.set_error(msg))
                    } else {
                        false
                    }
                }),
            _ => false,
        }
    }

    fn undo(&mut self) -> yewdux::prelude::Changed {
        if self.can_undo() {
            // FIXME: reset selected tile
            match self.nile.undo() {
                Ok(_) => true,
                Err(msg) => self.set_error(msg),
            }
        } else {
            false
        }
    }

    fn redo(&mut self) -> yewdux::prelude::Changed {
        if self.can_redo() {
            // FIXME: reset selected tile
            match self.nile.redo() {
                Ok(_) => true,
                Err(msg) => self.set_error(msg),
            }
        } else {
            false
        }
    }

    fn end_turn(&mut self) -> yewdux::prelude::Changed {
        if self.current_turn_tiles.is_empty() {
            false
        } else {
            match self.nile.end_turn() {
                Ok(_) => true,
                Err(msg) => self.set_error(msg),
            }
        }
    }

    fn cant_play(&mut self) -> yewdux::prelude::Changed {
        if self.current_turn_tiles.is_empty() {
            match self.nile.cant_play() {
                Ok(_) => true,
                Err(msg) => self.set_error(msg),
            }
        } else {
            false
        }
    }

    fn cpu_turn(&mut self) -> yewdux::prelude::Changed {
        if self.nile.current_player().is_cpu() {
            self.nile.take_cpu_turn().is_some()
        } else {
            false
        }
    }

    fn set_error(&mut self, msg: String) -> yewdux::prelude::Changed {
        update_if_changed(&mut self.modal, Some(Modal::Error(msg)))
    }

    fn set_end_of_game(&mut self, msg: String) -> yewdux::prelude::Changed {
        update_if_changed(&mut self.modal, Some(Modal::EndOfGame(msg)))
    }

    fn dismiss(&mut self) -> yewdux::prelude::Changed {
        update_if_changed(&mut self.modal, None)
    }
}

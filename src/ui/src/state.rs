use nile::{console, Coordinates, Engine, SelectedTile, TilePath};
use yewdux::prelude::{Reducer, ReducerStore};

use crate::components::utils::update_if_changed;

#[derive(Clone)]
pub struct State {
    /// Main game state
    pub nile: Engine,
    pub modal: Option<Modal>,
}

#[derive(Debug)]
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
    // CpuTurn,
    SetError(String),
    SetEndOfGame(String),
    Dismiss,
    None,
}

#[derive(Clone, PartialEq)]
pub enum Modal {
    Error(String),
    EndOfGame(String),
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectRackTile {
    pub rack_idx: u8,
}

pub type GameStore = ReducerStore<State>;

impl State {
    pub fn new_game(player_names: Vec<String>, cpu_player_count: u8) -> Self {
        Self {
            nile: Engine::new(player_names, cpu_player_count).unwrap(),
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
        match self.nile.selected_tile() {
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
        console::info(&format!("Received action: {:?}", action));
        match action {
            Action::SelectRackTile(select_rack_tile) => self
                .nile
                .select_rack_tile(select_rack_tile.rack_idx)
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::SelectBoardTile(coordinates) => self
                .nile
                .select_board_tile(coordinates)
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::PlaceTile(coordinates) => self
                .nile
                .place_tile(coordinates)
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::RotateSelectedTile(rotation) => self.rotate_selected_tile(rotation),
            Action::RemoveSelectedTile => self
                .nile
                .remove_selected_tile()
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::UpdateSelectedUniversalPath(tile_path) => self
                .nile
                .update_selected_universal_path(tile_path)
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::Undo => self
                .nile
                .undo()
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::Redo => self
                .nile
                .redo()
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::EndTurn => self
                .nile
                .end_turn()
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            Action::CantPlay => self
                .nile
                .cant_play()
                .map(|_| true)
                .unwrap_or_else(|e| self.set_error(e)),
            // Action::CpuTurn => self
            //     .nile
            //     .take_cpu_turn()
            //     .map(|_| true)
            //     .unwrap_or_else(|e| self.set_error(e)),
            Action::SetError(msg) => self.set_error(msg),
            Action::SetEndOfGame(msg) => self.set_end_of_game(msg),
            Action::Dismiss => self.dismiss(),
            Action::None => false,
        }
    }
}

impl State {
    fn rotate_selected_tile(&mut self, rotation: Rotation) -> yewdux::prelude::Changed {
        self.nile
            .selected_board_tile()
            .and_then(|coordinates| self.nile.board().cell(coordinates))
            .and_then(|cell| cell.tile())
            .map_or(false, |tile_placement| {
                let new_rotation = nile::ROTATIONS[(tile_placement.rotation() as i8 + {
                    if rotation == Rotation::Clockwise {
                        1
                    } else {
                        -1
                    }
                }) as usize
                    % nile::ROTATIONS.len()];
                console::debug("rotating");
                self.nile
                    .rotate_selected_tile(new_rotation)
                    .map_or_else(|msg| self.set_error(msg), |()| true)
            })
    }

    fn set_error(&mut self, msg: String) -> yewdux::prelude::Changed {
        console::error(&msg);
        update_if_changed(&mut self.modal, Some(Modal::Error(msg)))
    }

    fn set_end_of_game(&mut self, msg: String) -> yewdux::prelude::Changed {
        update_if_changed(&mut self.modal, Some(Modal::EndOfGame(msg)))
    }

    fn dismiss(&mut self) -> yewdux::prelude::Changed {
        update_if_changed(&mut self.modal, None)
    }
}

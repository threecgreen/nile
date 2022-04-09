use std::rc::Rc;

use nile::{console, Coordinates, Engine, Player, SelectedTile, TilePath, TilePathType};
use yewdux::{prelude::Reducer, store::Store};

use crate::components::utils::update_if_changed;

/// Shared state for a game of nile.
#[derive(Clone)]
pub struct State {
    /// Main game state
    pub nile: Engine,
    /// Modal state for displaying errors and end-of-game message
    pub modal: Option<Modal>,
}

/// A message when [`State`] has changed.
pub struct UpdateStateMsg(pub Rc<State>);

pub type Dispatch = yewdux::dispatch::Dispatch<State>;

#[derive(Debug)]
pub enum Action {
    NewGame(NewGameOptions),
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
    Dismiss,
}

#[derive(Clone, PartialEq)]
pub enum Modal {
    Error(String),
    EndOfGame(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewGameOptions {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
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

impl Store for State {
    fn new() -> Self {
        Self::new_game(vec![String::default()], 1)
    }

    fn should_notify(&self, _old: &Self) -> bool {
        true
    }
}

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
        matches!(self.nile.selected_tile(), Some(SelectedTile::Board(_)))
    }

    pub fn selected_is_universal(&self) -> bool {
        self.nile
            .selected_board_tile()
            .and_then(|coordinates| self.nile.board().cell(coordinates))
            .and_then(|cell| cell.tile())
            .map_or(false, |c| {
                matches!(c.tile_path_type(), TilePathType::Universal(_))
            })
    }
}

impl Reducer<State> for Action {
    fn apply(self, mut state_rc: Rc<State>) -> Rc<State> {
        // copy on write
        let state = Rc::make_mut(&mut state_rc);
        // console::info(&format!("Received action: {:?}", action));
        match self {
            Action::NewGame(NewGameOptions {
                player_names,
                cpu_player_count,
            }) => {
                state.nile = Engine::new(player_names, cpu_player_count).expect("nile engine");
            }
            Action::SelectRackTile(select_rack_tile) => {
                if let Err(e) = state.nile.select_rack_tile(select_rack_tile.rack_idx) {
                    state.set_error(e);
                }
            }
            Action::SelectBoardTile(coordinates) => {
                if let Err(e) = state.nile.select_board_tile(coordinates) {
                    state.set_error(e);
                }
            }
            Action::PlaceTile(coordinates) => {
                if let Err(e) = state.nile.place_tile(coordinates) {
                    state.set_error(e);
                }
            }
            Action::RotateSelectedTile(rotation) => {
                state.rotate_selected_tile(rotation);
            }
            Action::RemoveSelectedTile => {
                if let Err(e) = state.nile.remove_selected_tile() {
                    state.set_error(e);
                }
            }
            Action::UpdateSelectedUniversalPath(tile_path) => {
                if let Err(e) = state.nile.update_selected_universal_path(tile_path) {
                    state.set_error(e);
                }
            }
            Action::Undo => {
                if let Err(e) = state.nile.undo() {
                    state.set_error(e);
                }
            }
            Action::Redo => {
                if let Err(e) = state.nile.redo() {
                    state.set_error(e);
                }
            }
            Action::EndTurn => {
                state.end_turn();
            }
            Action::CantPlay => {
                state.cant_play();
            }
            Action::Dismiss => {
                state.dismiss();
            }
        };
        state_rc
    }
}

impl State {
    fn rotate_selected_tile(&mut self, rotation: Rotation) -> bool {
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
                self.nile
                    .rotate_selected_tile(new_rotation)
                    .map_or_else(|msg| self.set_error(msg), |()| true)
            })
    }

    fn end_turn(&mut self) -> bool {
        self.nile
            .end_turn()
            .map(|has_ended| {
                if has_ended {
                    self.set_end_of_game();
                }
                true
            })
            .unwrap_or_else(|e| self.set_error(e))
    }

    fn cant_play(&mut self) -> bool {
        self.nile
            .cant_play()
            .map(|has_ended| {
                if has_ended {
                    self.set_end_of_game();
                }
                true
            })
            .unwrap_or_else(|e| self.set_error(e))
    }

    fn set_end_of_game(&mut self) -> bool {
        let winning_score = self
            .nile
            .players()
            .iter()
            .fold(i16::MIN, |acc, p| i16::max(acc, p.total_score()));
        let winners: Vec<&Player> = self
            .nile
            .players()
            .iter()
            .filter(|p| p.total_score() == winning_score)
            .collect();
        let msg = match winners.len() {
            0 => unreachable!(),
            1 => format!("{} has won", winners[0].name()),
            _ => format!(
                "{} tied",
                winners
                    .iter()
                    .map(|p| p.name())
                    .collect::<Vec<&str>>()
                    .join(", ")
            ),
        };
        update_if_changed(&mut self.modal, Some(Modal::EndOfGame(msg)))
    }

    fn set_error(&mut self, msg: String) -> bool {
        console::error(&msg);
        update_if_changed(&mut self.modal, Some(Modal::Error(msg)))
    }

    fn dismiss(&mut self) -> bool {
        update_if_changed(&mut self.modal, None)
    }
}

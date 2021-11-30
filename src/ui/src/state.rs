use nile::{console, Coordinates, Engine, Player, SelectedTile, TilePath, TilePathType};
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
    // CpuTurn,
    // SetError(String),
    // SetEndOfGame(String),
    Dismiss,
    // None,
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

impl Reducer for State {
    type Action = Action;

    fn new() -> Self {
        Self::new_game(vec![String::default()], 1)
    }

    fn reduce(&mut self, action: Self::Action) -> yewdux::prelude::Changed {
        console::info(&format!("Received action: {:?}", action));
        match action {
            Action::NewGame(NewGameOptions {
                player_names,
                cpu_player_count,
            }) => {
                self.nile = Engine::new(player_names, cpu_player_count).expect("nile engine");
                true
            }
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
            Action::EndTurn => self.end_turn(),
            Action::CantPlay => self.cant_play(),
            // Action::CpuTurn => self
            //     .nile
            //     .take_cpu_turn()
            //     .map(|_| true)
            //     .unwrap_or_else(|e| self.set_error(e)),
            // Action::SetError(msg) => self.set_error(msg),
            // Action::SetEndOfGame(msg) => self.set_end_of_game(msg),
            Action::Dismiss => self.dismiss(),
            // Action::None => false,
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
                self.nile
                    .rotate_selected_tile(new_rotation)
                    .map_or_else(|msg| self.set_error(msg), |()| true)
            })
    }

    fn end_turn(&mut self) -> yewdux::prelude::Changed {
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

    fn cant_play(&mut self) -> yewdux::prelude::Changed {
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

    fn set_end_of_game(&mut self) -> yewdux::prelude::Changed {
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

    fn set_error(&mut self, msg: String) -> yewdux::prelude::Changed {
        console::error(&msg);
        update_if_changed(&mut self.modal, Some(Modal::Error(msg)))
    }

    fn dismiss(&mut self) -> yewdux::prelude::Changed {
        update_if_changed(&mut self.modal, None)
    }
}

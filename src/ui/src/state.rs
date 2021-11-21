use nile::{Coordinates, Nile, TilePath, TilePlacement, TurnScore};
use std::collections::HashSet;
use yewdux::prelude::{Reducer, ReducerStore};

#[derive(Clone)]
pub struct State {
    /// Main game state
    pub nile: nile::Nile,
    /// Used for determining if placed tile is movable, rotatable, etc.
    pub current_turn_tiles: HashSet<Coordinates>,
    pub selected_tile: Option<SelectedTile>,
    pub modal: Option<Modal>,
}

#[derive(Clone)]
pub enum SelectedTile {
    Rack(SelectRackTile),
    Board(Coordinates),
}

#[derive(Clone)]
pub enum Modal {
    Error(String),
    EndOfGame(String),
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
            Action::SelectRackTile(SelectRackTile { rack_idx }) => {
                let current_player = self.nile.current_player();
                // validate
                if let Some(tile) = current_player.tiles().get(rack_idx as usize) {
                    // store
                    self.selected_tile = Some(SelectedTile::Rack(SelectRackTile { rack_idx }));
                    true
                } else {
                    false
                }
            }
            Action::SelectBoardTile(Coordinates(x, y)) => false,
            _ => false,
        }
    }
}

pub enum Action {
    SelectRackTile(SelectRackTile),
    SelectBoardTile(Coordinates),
    /// place a tile on the board. It will be moved from its previous location
    PlaceTile(Coordinates),
    RotateSelectedTile(Rotation),
    RemoveTile,
    UpdateSelectedUniversalPath(TilePath),
    Undo,
    Redo,
    EndTurn,
    CantPlay,
    CpuTurn,
    SetError(String),
    SetEndOfGame(String),
    Dismiss,
}

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone)]
pub struct SelectRackTile {
    rack_idx: u8,
}

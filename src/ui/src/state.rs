use nile::{CPUTurnUpdate, Coordinates, EndTurnUpdate, Nile, TilePath, TilePlacement, TurnScore};
use std::{collections::HashSet, hash::Hash};
use yewdux::prelude::{Reducer, ReducerStore};

#[derive(Clone)]
pub struct State {
    /// Main game state
    nile: nile::Nile,
    /// Used for determining if placed tile is movable, rotatable, etc.
    current_turn_tiles: HashSet<Coordinates>,
    selected_tile: Option<SelectedTile>,
    modal: Option<Modal>,
}

#[derive(Clone)]
enum SelectedTile {
    Rack(RackTile),
    Board(Coordinates),
}

#[derive(Clone)]
enum Modal {
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
}

impl Reducer for State {
    type Action = Action;

    fn new() -> Self {
        Self::new_game(Vec::default(), 0)
    }

    fn reduce(&mut self, action: Self::Action) -> yewdux::prelude::Changed {
        match action {
            Action::SelectRackTile(rack_tile) => {
                let current_player = self.nile.current_player();
                // validate
                if let Some(tile) = current_player.tiles().get(rack_tile.rack_idx as usize) {
                    // store
                    self.selected_tile = Some(SelectedTile::Rack(rack_tile));
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
    SelectRackTile(RackTile),
    SelectBoardTile(Coordinates),
    PlaceTile(PlaceTile),
    RotateTile(TilePlacement),
    RemoveTile(RemoveTile),
    UpdateUniversalPath(UpdateUniversalPath),
    MoveTile(MoveTile),
    Undo,
    Redo,
    EndTurn(EndTurnUpdate),
    CpuTurn(CPUTurnUpdate),
    SetError(String),
    SetEndOfGame(String),
    Dismiss,
}

#[derive(Clone)]
struct RackTile {
    tile_path: TilePath,
    is_universal: bool,
    rack_idx: u8,
}

struct PlaceTile {
    dragged_tile: RackTile,
    tile_placement: TilePlacement,
    score: TurnScore,
}

struct RemoveTile {
    coordinates: Coordinates,
    score: TurnScore,
}

struct UpdateUniversalPath {
    coordinates: Coordinates,
    tile_placement: TilePlacement,
}

struct MoveTile {
    old_coordinates: Coordinates,
    new_coordinates: Coordinates,
    tile_placement: TilePlacement,
    score: TurnScore,
}

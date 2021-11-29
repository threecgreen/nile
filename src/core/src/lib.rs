mod ai;
mod board;
pub mod console;
mod log;
mod nile;
mod path;
mod player;
mod score;
mod tile;

#[macro_use]
extern crate wasm_bindgen;

pub use crate::board::{Board, Cell, TilePlacement, BOARD_DIM};
pub use crate::nile::{CPUTurnUpdate, EndTurnUpdate, Engine, Nile, SelectedTile};
pub use crate::path::{TilePath, TilePathType};
pub use crate::player::{Player, TileArray};
pub use crate::score::TurnScore;
pub use crate::tile::{Coordinates, Rotation, Tile, ROTATIONS};

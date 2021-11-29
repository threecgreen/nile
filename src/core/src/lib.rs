mod ai;
mod board;
mod log;
mod nile;
mod path;
mod player;
mod score;
mod tile;

pub use crate::board::{Board, Cell, TilePlacement, BOARD_DIM};
pub use crate::nile::{CPUTurnUpdate, EndTurnUpdate, Engine, Nile, SelectedTile};
pub use crate::path::{TilePath, TilePathType};
pub use crate::player::{Player, TileArray};
pub use crate::score::TurnScore;
pub use crate::tile::{Coordinates, Rotation, Tile, ROTATIONS};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

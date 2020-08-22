mod board;
mod log;
mod nile;
mod path;
mod player;
mod score;
mod tile;
mod wasm_api;

pub use crate::board::{Board, Cell, TilePlacement};
pub use crate::nile::Nile;
pub use crate::path::{
    wasm::{tile_path_to_tile, TilePathType},
    TilePath,
};
pub use crate::player::Player;
pub use crate::tile::{Coordinates, Rotation, Tile};
pub use crate::wasm_api::WasmNile;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

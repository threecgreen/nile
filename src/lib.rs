mod board;
mod log;
mod nile;
mod path;
mod player;
mod score;
mod tile;
mod wasm_api;

pub use board::{Board, Cell, TilePlacement};
pub use nile::Nile;
pub use path::{
    wasm::{tile_path_to_tile, TilePathType},
    TilePath,
};
pub use player::Player;
pub use tile::{Coordinates, Rotation, Tile};
pub use wasm_api::WasmNile;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

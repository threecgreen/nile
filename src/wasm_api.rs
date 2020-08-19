use crate::board::Board;
use crate::nile::Nile;
use crate::player::Player;
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation, Tile};

use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Wrapper for access from wasm that handles the serialization and type
/// conversion necessary for communicating between JS and WebAssembly
#[wasm_bindgen]
pub struct WasmNile {
    nile: Nile,
}

#[wasm_bindgen]
impl WasmNile {
    /// Create a new game
    #[wasm_bindgen(constructor)]
    pub fn new(player_names: Array) -> Result<WasmNile, JsValue> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let iterator =
            js_sys::try_iter(&player_names)?.ok_or_else(|| "Need to pass array of strings")?;
        let mut player_names = Vec::<String>::new();
        for name in iterator {
            // Bubble up iteration errors
            let name = name?;
            if let Some(name) = name.as_string() {
                player_names.push(name);
            }
        }
        match Nile::new(player_names) {
            Ok(nile) => Ok(WasmNile { nile }),
            Err(err) => Err(err.into()),
        }
    }

    /// Get full game board. Should only be used on initialization
    pub fn board(&self) -> Board {
        self.nile.board().to_owned()
    }

    /// @returns an array of `Player`
    pub fn players(&self) -> Array {
        self.nile
            .players()
            .to_owned()
            .into_iter()
            .map(JsValue::from)
            .collect()
    }

    pub fn current_turn_player_id(&self) -> usize {
        self.nile.current_turn()
    }

    pub fn can_undo(&self) -> bool {
        self.nile.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.nile.can_redo()
    }

    pub fn place_tile(
        &mut self,
        tile: Tile,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<TurnScore, JsValue> {
        self.nile
            .place_tile(tile, coordinates, rotation)
            .map_err(JsValue::from)
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<(), JsValue> {
        self.nile
            .rotate_tile(coordinates, rotation)
            .map_err(JsValue::from)
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Result<TurnScore, JsValue> {
        self.nile.remove_tile(coordinates).map_err(JsValue::from)
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> Result<TurnScore, JsValue> {
        self.nile
            .move_tile(old_coordinates, new_coordinates)
            .map_err(JsValue::from)
    }

    pub fn undo(&mut self) -> Result<Option<TurnScore>, JsValue> {
        self.nile.undo().map_err(JsValue::from)
    }

    pub fn redo(&mut self) -> Result<Option<TurnScore>, JsValue> {
        self.nile.redo().map_err(JsValue::from)
    }
}

#[wasm_bindgen]
impl Player {
    pub fn get_tiles(&self) -> Array {
        self.tiles()
            .into_iter()
            .map(|t| JsValue::from_serde(t).unwrap())
            .collect()
    }
}

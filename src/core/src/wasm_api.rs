use crate::board::Board;
use crate::nile::{CPUTurnUpdate, EndTurnUpdate, Nile};
use crate::path::{self, TilePath};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation};

use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Wrapper for access from wasm that handles the serialization and type
/// conversion necessary for communicating between JS and WebAssembly
#[wasm_bindgen]
pub struct WasmNile(Nile);

#[wasm_bindgen]
impl WasmNile {
    /// Create a new game
    #[wasm_bindgen(constructor)]
    pub fn new(player_names: Array, ai_count: usize) -> Result<WasmNile, JsValue> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let iterator = js_sys::try_iter(&player_names)?.ok_or("Need to pass array of strings")?;
        let mut player_names = Vec::<String>::new();
        for name in iterator {
            // Bubble up iteration errors
            let name = name?;
            if let Some(name) = name.as_string() {
                player_names.push(name);
            }
        }
        match Nile::new(player_names, ai_count) {
            Ok(nile) => Ok(WasmNile(nile)),
            Err(err) => Err(err.into()),
        }
    }

    /// Get full game board. Should only be used on initialization
    pub fn board(&self) -> Board {
        self.0.board().to_owned()
    }

    /// @returns an array of `Player`
    pub fn players(&self) -> Array {
        self.0
            .players()
            .to_owned()
            .into_iter()
            .map(JsValue::from)
            .collect()
    }

    pub fn current_turn_player_id(&self) -> usize {
        self.0.current_turn()
    }

    pub fn can_undo(&self) -> bool {
        self.0.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.0.can_redo()
    }

    pub fn has_ended(&self) -> bool {
        self.0.has_ended()
    }

    pub fn place_tile(
        &mut self,
        tile_placement_type: path::wasm::TilePathType,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<TurnScore, JsValue> {
        self.0
            .place_tile(tile_placement_type.into(), coordinates, rotation)
            .map_err(JsValue::from)
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<(), JsValue> {
        self.0
            .rotate_tile(coordinates, rotation)
            .map_err(JsValue::from)
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Result<TurnScore, JsValue> {
        self.0.remove_tile(coordinates).map_err(JsValue::from)
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> Result<TurnScore, JsValue> {
        self.0
            .move_tile(old_coordinates, new_coordinates)
            .map_err(JsValue::from)
    }

    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        tile_path: TilePath,
    ) -> Result<(), JsValue> {
        self.0
            .update_universal_path(coordinates, tile_path)
            .map_err(JsValue::from)
    }

    pub fn end_turn(&mut self) -> Result<EndTurnUpdate, JsValue> {
        self.0.end_turn().map_err(JsValue::from)
    }

    pub fn cant_play(&mut self) -> Result<EndTurnUpdate, JsValue> {
        self.0.cant_play().map_err(JsValue::from)
    }

    pub fn undo(&mut self) -> Result<Option<TurnScore>, JsValue> {
        self.0.undo().map_err(JsValue::from)
    }

    pub fn redo(&mut self) -> Result<Option<TurnScore>, JsValue> {
        self.0.redo().map_err(JsValue::from)
    }

    pub fn take_cpu_turn(&mut self) -> Option<CPUTurnUpdate> {
        self.0.take_cpu_turn()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use wasm_bindgen_test::*;

    fn setup() -> WasmNile {
        let player_names: Array = vec![JsValue::from("Carter")].iter().collect();
        WasmNile::new(player_names, 1).unwrap()
    }

    // #[wasm_bindgen_test]
    fn test_end_of_game() {
        let mut target = setup();
        let placement_res = target
            .place_tile(
                path::wasm::TilePathType::normal(TilePath::Straight),
                Coordinates(14, 21),
                Rotation::None,
            )
            .unwrap();
        assert_eq!(placement_res.score(), 10 + 100 /* bonus */);
        let res = target.end_turn().unwrap();
        assert!(res.game_has_ended);
        assert!(target.take_cpu_turn().is_none())
    }

    #[wasm_bindgen_test]
    fn take_cpu_turn_for_human_fails() {
        let mut target = setup();
        assert!(matches!(target.take_cpu_turn(), None));
    }
}

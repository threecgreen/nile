use crate::board::Board;
use crate::nile::Nile;

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

    /// Progress the game in some manner
    pub fn handle_event(&mut self, event: &JsValue) -> Result<(), JsValue> {
        let event = event.into_serde().map_err(|_| "Invalid event")?;

        self.nile.handle_event(event).map_err(|e| e.into())
    }
}

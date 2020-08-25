use crate::path::{self, TilePath, TilePathType};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation};

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TilePlacement {
    tile_path_type: TilePathType,
    rotation: Rotation,
}

#[wasm_bindgen]
impl TilePlacement {
    pub fn get_tile_path_type(&self) -> path::wasm::TilePathType {
        path::wasm::TilePathType::from(self.tile_path_type.clone())
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }
}

/// Accessor methods for rust. The fields can't be public because
/// `TilePlacementType` isn't representable in wasm
impl TilePlacement {
    pub fn new(tile_path_type: TilePathType, rotation: Rotation) -> Self {
        Self {
            tile_path_type,
            rotation,
        }
    }

    pub fn tile_path_type(&self) -> &TilePathType {
        &self.tile_path_type
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Cell {
    bonus: i16,
    tile: Option<TilePlacement>,
}

#[wasm_bindgen]
impl Cell {
    pub fn tile(&self) -> Option<TilePlacement> {
        self.tile.clone()
    }

    pub fn bonus(&self) -> i16 {
        self.bonus
    }
}

impl Cell {
    pub fn with_bonus(bonus: i16) -> Self {
        Self { bonus, tile: None }
    }

    pub fn set_tile(&mut self, tile: TilePlacement) -> TurnScore {
        self.tile = Some(tile);
        self.score()
    }

    pub fn remove_tile(&mut self) -> Option<(TilePlacement, TurnScore)> {
        let old_score = self.score();
        self.tile.take().map(|tp| (tp, -old_score))
    }

    pub fn is_empty(&self) -> bool {
        self.tile.is_none()
    }

    pub fn score(&self) -> TurnScore {
        let tile_score = self
            .tile
            .as_ref()
            .map(|t| t.tile_path_type.score())
            .unwrap_or(0);
        if self.bonus >= 0 {
            TurnScore::new(self.bonus + tile_score, 0)
        } else {
            TurnScore::new(tile_score, self.bonus)
        }
    }

    pub fn update_universal_path(&mut self, tile_path: TilePath) -> Result<TilePath, String> {
        let tile_placement = self
            .tile
            .as_mut()
            .ok_or_else(|| "Cell is empty".to_owned())?;
        let old_tile_path = match tile_placement.tile_path_type {
            TilePathType::Normal(_) => {
                return Err("Cell doesn't contain a universal tile".to_owned());
            }
            TilePathType::Universal(tp) => tp,
        };
        tile_placement.tile_path_type = TilePathType::Universal(tile_path);

        Ok(old_tile_path)
    }
}

/// The board is 21x21 plus a special end of game column
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Board {
    cells: Vec<Cell>,
    end_of_game_cells: Vec<Cell>,
}

static BOARD_SIZE: usize = 21;

#[wasm_bindgen]
impl Board {
    pub fn height(&self) -> usize {
        BOARD_SIZE
    }

    pub fn width(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_cell(&self, row: i8, column: i8) -> Cell {
        self.cells[self.get_index(Coordinates(row, column))].clone()
    }
}

macro_rules! hash_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

impl Board {
    pub fn new() -> Board {
        let bonus_order: Vec<i16> = vec![50, 50, 50, 50, 75, 75, 100, 100, 150, 200, 500];
        let bonuses: HashMap<(usize, usize), i16> = hash_map!(
            (1, 6) => -80,
            (1, 10) => -100,
            (1, 14) => -120,
            (2, 2) => -40,
            (2, 3) => -40,
            (2, 17) => -160,
            (2, 18) => -160,
            (3, 2) => -40,
            (3, 8) => 140,
            (3, 12) => 220,
            (3, 18) => -160,
            (4, 2) => -40,
            (4, 3) => -40,
            (4, 17) => -160,
            (4, 18) => -160,
            (5, 6) => -80,
            (5, 10) => -100,
            (5, 14) => -120,
            (7, 2) => 60,
            (7, 10) => 240,
            (7, 18) => 250,
            (8, 7) => 240,
            (8, 13) => 240,
            (9, 5) => -60,
            (9, 9) => -40,
            (9, 11) => -40,
            (9, 15) => -140,
            (10, 10) => -60,
            (10, 18) => -160
        );
        let cells: Vec<Cell> = (0..BOARD_SIZE * BOARD_SIZE)
            .into_iter()
            .map(|i| {
                let row = {
                    let row = i / BOARD_SIZE;
                    // Board is reflected across horizontal axis
                    if row > 10 {
                        BOARD_SIZE - 1 - row
                    } else {
                        row
                    }
                };
                let col = i % BOARD_SIZE;
                bonuses
                    .get(&(row, col))
                    .map(|b| Cell::with_bonus(*b))
                    .unwrap_or_default()
            })
            .collect();
        Self {
            cells,
            // Symmetrical
            end_of_game_cells: bonus_order
                .iter()
                .chain(bonus_order.iter().rev().skip(1))
                .map(|b| Cell::with_bonus(*b))
                .collect(),
        }
    }

    fn get_index(&self, coordinates: Coordinates) -> usize {
        let row = coordinates.0 as usize;
        let column = coordinates.1 as usize;
        row * self.width() + column
    }

    pub fn place_tile(
        &mut self,
        coordinates: Coordinates,
        tile_placement: TilePlacement,
    ) -> Result<TurnScore, String> {
        let idx = self.get_index(coordinates);
        if self.cells[idx].is_empty() {
            Ok(self.cells[idx].set_tile(tile_placement))
        } else {
            Err("There's already a tile there".to_owned())
        }
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<(), String> {
        let idx = self.get_index(coordinates);
        if let Some(ref mut tile) = self.cells[idx].tile {
            tile.rotation = rotation;
            Ok(())
        } else {
            Err("Cell is empty".to_owned())
        }
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Option<(TilePlacement, TurnScore)> {
        let idx = self.get_index(coordinates);
        self.cells[idx].remove_tile()
    }

    /// Returns the old `TilePath`
    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        tile_path: TilePath,
    ) -> Result<TilePath, String> {
        let idx = self.get_index(coordinates);
        self.cells[idx].update_universal_path(tile_path)
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> Result<TurnScore, String> {
        let (tile_placement, removal_score) = self
            .remove_tile(old_coordinates)
            .ok_or_else(|| "Can't move tile when there's no tile at old coordinates".to_owned())?;
        let placement_score = self
            .place_tile(new_coordinates, tile_placement.clone())
            .map_err(|e| {
                // Try to replace the tile
                self.place_tile(old_coordinates, tile_placement)
                    .expect("should be able to replace tile");
                e
            })?;
        // `removal_score` should already be negative to revert the score effect of placement
        Ok(placement_score + removal_score)
    }

    pub fn has_tile(&self, coordinates: Coordinates) -> bool {
        let idx = self.get_index(coordinates);
        self.cells[idx].is_empty()
    }
}

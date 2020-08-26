use crate::log::TilePlacementEvent;
use crate::path::{self, eval_placement, Offset, TilePath, TilePathType};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation};

use std::collections::{HashMap, HashSet};
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

// TODO: figure out how to know all the placements from this turn. Probably log

/// The board is 21x21 plus a special end of game column
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Board {
    last_placement: (Coordinates, Offset),
    cells: Vec<Cell>,
    end_of_game_cells: Vec<Cell>,
}

static BOARD_SIZE: usize = 21;

pub mod wasm {
    use super::{Board, Cell, Coordinates, BOARD_SIZE};

    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Board {
        pub fn height(&self) -> usize {
            BOARD_SIZE
        }

        pub fn width(&self) -> usize {
            BOARD_SIZE
        }

        pub fn get_cell(&self, row: i8, column: i8) -> Result<Cell, JsValue> {
            self.cell(Coordinates(row, column))
                .map(|cell| cell.clone())
                .ok_or_else(|| JsValue::from("Invalid coordinates"))
        }
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
            // Start arrow placement and offset
            last_placement: (Coordinates(10, -1), Offset(0, 1)),
            cells,
            // Symmetrical
            end_of_game_cells: bonus_order
                .iter()
                .chain(bonus_order.iter().rev().skip(1))
                .map(|b| Cell::with_bonus(*b))
                .collect(),
        }
    }

    pub fn cell(&self, coordinates: Coordinates) -> Option<&Cell> {
        let width = self.width();
        if coordinates.1 as usize == self.width() {
            self.end_of_game_cells.get(coordinates.0 as usize)
        } else {
            let row = coordinates.0 as usize;
            let column = coordinates.1 as usize;
            self.cells.get(row * width + column)
        }
    }

    fn get_mut_cell(&mut self, coordinates: Coordinates) -> Option<&mut Cell> {
        let width = self.width();
        if coordinates.1 as usize == self.width() {
            self.end_of_game_cells.get_mut(coordinates.0 as usize)
        } else {
            let row = coordinates.0 as usize;
            let column = coordinates.1 as usize;
            self.cells.get_mut(row * width + column)
        }
    }

    pub fn place_tile(
        &mut self,
        coordinates: Coordinates,
        tile_placement: TilePlacement,
    ) -> Result<TurnScore, String> {
        match self.get_mut_cell(coordinates) {
            Some(cell) if cell.is_empty() => Ok(cell.set_tile(tile_placement)),
            Some(_) => Err("There's already a tile there".to_owned()),
            None => Err("Invalid coordinates".to_owned()),
        }
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<(), String> {
        let cell = self
            .get_mut_cell(coordinates)
            .ok_or_else(|| "Invalid coordinates".to_owned())?;
        if let Some(ref mut tile) = cell.tile {
            tile.rotation = rotation;
            Ok(())
        } else {
            Err("Cell is empty".to_owned())
        }
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Option<(TilePlacement, TurnScore)> {
        match self.get_mut_cell(coordinates) {
            Some(cell) => cell.remove_tile(),
            None => None,
        }
    }

    /// Returns the old `TilePath`
    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        tile_path: TilePath,
    ) -> Result<TilePath, String> {
        match self.get_mut_cell(coordinates) {
            Some(cell) => cell.update_universal_path(tile_path),
            None => Err("Invalid coordinates".to_owned()),
        }
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
        self.cell(coordinates)
            .map(|cell| cell.is_empty())
            .unwrap_or_default()
    }

    /// Called as part of end of turn
    pub fn validate_turns_moves(
        &mut self,
        mut turn_coordinates: HashSet<Coordinates>,
    ) -> Result<(), String> {
        let mut last_placement = self.last_placement.clone();
        while !turn_coordinates.is_empty() {
            let next_coordinates = last_placement.0 + last_placement.1;
            let cell = self
                .cell(next_coordinates)
                .ok_or_else(|| format!("Invalid coordinates {:?}", next_coordinates))?;
            let tile = cell
                .tile()
                .ok_or_else(|| format!("Non-contiguous path. No tile at {:?}", next_coordinates))?;
            last_placement = eval_placement(
                last_placement,
                &TilePlacementEvent {
                    tile_path_type: tile.tile_path_type,
                    rotation: tile.rotation,
                    coordinates: last_placement.0,
                },
            )?;
            if !turn_coordinates.remove(&last_placement.0) {
                return Err("Can't reuse a tile from another turn".to_owned());
            }
        }
        self.last_placement = last_placement;
        Ok(())
    }
}

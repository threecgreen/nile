use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation, TilePlacement};

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Cell {
    bonus: i16,
    tile: Option<TilePlacement>,
}

#[wasm_bindgen]
impl Cell {
    pub fn tile(&self) -> Option<TilePlacement> {
        self.tile
    }

    pub fn bonus(&self) -> i16 {
        self.bonus
    }

    pub fn update_tile(&mut self, tile: Option<TilePlacement>) {
        self.tile = tile;
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
        let tile_score = self.tile.map(|t| t.tile.score()).unwrap_or(0);
        if self.bonus >= 0 {
            TurnScore::new(self.bonus + tile_score, 0)
        } else {
            TurnScore::new(tile_score, self.bonus)
        }
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
                let row = i / BOARD_SIZE;
                let col = i % BOARD_SIZE;
                // Board is reflected across horizontal axis
                let adj_row = if row > 10 { BOARD_SIZE - 1 - row } else { row };
                bonuses
                    .get(&(adj_row, col))
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
        let row = coordinates.1 as usize;
        let column = coordinates.0 as usize;
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

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Option<(TilePlacement, TurnScore)> {
        let idx = self.get_index(coordinates);
        self.cells[idx].remove_tile()
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

    pub fn has_tile(&self, coordinates: Coordinates) -> bool {
        let idx = self.get_index(coordinates);
        self.cells[idx].is_empty()
    }
}

use crate::tile::{Coordinates, Tile, TilePlacement, Rotation};

#[derive(Clone, Debug, Default)]
pub struct Cell {
    bonus: i16,
    tile: Option<TilePlacement>,
}

impl Cell {
    pub fn with_bonus(bonus: i16) -> Self {
        Self { bonus, tile: None }
    }

    pub fn set_tile(&mut self, tile: TilePlacement) {
        self.tile = Some(tile);
    }

    pub fn remove_tile(&mut self) -> Option<TilePlacement> {
        self.tile.take()
    }

    pub fn is_empty(&self) -> bool {
        self.tile.is_some()
    }

    pub fn score(&self) -> i16 {
        self.bonus + self.tile.map(|t| t.tile.score()).unwrap_or(0)
    }
}

/// The board is 21x21 plus a special end of game column
#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    end_of_game_cells: Vec<Cell>,
}

static BOARD_SIZE: usize = 21;

impl Board {
    pub fn new() -> Board {
        let bonus_order: Vec<i16> = vec![50, 50, 50, 50, 75, 75, 100, 100, 150, 200, 500];
        Self {
            // TODO: set penalties and bonuses
            cells: vec![Cell::default(); BOARD_SIZE * BOARD_SIZE],
            // Symmetrical
            end_of_game_cells: bonus_order
                .iter()
                .chain(bonus_order.iter().rev().skip(1))
                .map(|b| Cell::with_bonus(*b))
                .collect(),
        }
    }

    pub fn height(&self) -> usize {
        BOARD_SIZE
    }

    pub fn width(&self) -> usize {
        BOARD_SIZE
    }

    fn get_index(&self, coordinates: Coordinates) -> usize {
        let row = coordinates.1 as usize;
        let column = coordinates.0 as usize;
        row * self.width() + column
    }

    pub fn get_cell(&self, coordinates: Coordinates) -> &Cell {
        &self.cells[self.get_index(coordinates)]
    }

    pub fn place_tile(&mut self, coordinates: Coordinates, tile_placement: TilePlacement)  {
        let idx = self.get_index(coordinates);
        // TODO: check if empty or should that be handled by engine
        self.cells[idx].set_tile(tile_placement)
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Option<TilePlacement> {
        let idx = self.get_index(coordinates);
        self.cells[idx].remove_tile()
    }

    pub fn rotate_tile(&mut self, coordinates: Coordinates, rotation: Rotation) {
        let idx = self.get_index(coordinates);
        if let Some(ref mut tile) = self.cells[idx].tile {
            tile.rotation = rotation;
        }
    }
}

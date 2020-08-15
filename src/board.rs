use crate::tile::Tile;

#[derive(Clone, Debug, Default)]
pub struct Cell {
    bonus: i16,
    tile: Option<Tile>,
}

impl Cell {
    pub fn with_bonus(bonus: i16) -> Self {
        Self { bonus, tile: None }
    }

    pub fn set_tile(&mut self, tile: Tile) {
        self.tile = Some(tile);
    }

    pub fn is_empty(&self) -> bool {
        self.tile.is_some()
    }

    pub fn score(&self) -> i16 {
        self.bonus + self.tile.map(|t| t.score()).unwrap_or(0)
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

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width() + column
    }

    pub fn get_cell(&self, row: usize, column: usize) -> &Cell {
        &self.cells[self.get_index(row, column)]
    }

    pub fn place_tile(&mut self, row: usize, column: usize, tile: Tile) {
        let idx = self.get_index(row, column);
        // TODO: check if empty or should that be handled by engine
        self.cells[idx].set_tile(tile)
    }
}

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

/// A unique location on the board
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinates(pub i8, pub i8);

#[wasm_bindgen]
impl Coordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(row: i8, column: i8) -> Self {
        Self(row, column)
    }
}

/// A game piece that can be placed on the board
#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    /// ```text
    ///
    /// ---
    ///
    /// ```.clone()
    Straight,
    /// ```text
    /// \
    ///  \
    ///   \
    /// ```
    Diagonal,
    /// ```text
    ///   |
    ///  /
    /// -
    /// ```
    Center90,
    /// ```text
    ///
    ///  -
    /// / \
    /// ```
    Corner90,
    /// ```text
    ///   /
    /// --
    ///
    /// ```
    Left45,
    /// ```text
    ///
    /// --
    ///   \
    /// ```
    Right45,
    /// ```text
    /// \
    ///  \
    /// ---
    /// ```
    Left135,
    /// ```text
    /// ---
    //   /
    //  /
    /// ```
    Right135,
    /// Can represent any one of the other `Tile` variants
    Universal,
}

impl Tile {
    pub fn score(self) -> i16 {
        match self {
            Tile::Straight | Tile::Diagonal | Tile::Center90 | Tile::Corner90 => 10,
            Tile::Left45 | Tile::Right45 => 8,
            Tile::Left135 | Tile::Right135 => 5,
            Tile::Universal => 35,
        }
    }
}

pub mod wasm {
    use super::*;

    #[wasm_bindgen]
    pub fn tile_score(tile: Tile) -> i16 {
        tile.score()
    }
}

/// Holds tiles that can still be drawn by a player
#[derive(Debug)]
pub struct TileBox {
    tiles: VecDeque<Tile>,
    rng: ThreadRng,
}

impl TileBox {
    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity(104);
        // Frequencies from the original game board
        Self::push_n(&mut tiles, Tile::Left135, 10);
        Self::push_n(&mut tiles, Tile::Center90, 10);
        Self::push_n(&mut tiles, Tile::Left45, 10);
        Self::push_n(&mut tiles, Tile::Straight, 20);
        Self::push_n(&mut tiles, Tile::Right45, 10);
        Self::push_n(&mut tiles, Tile::Right135, 10);
        Self::push_n(&mut tiles, Tile::Diagonal, 20);
        Self::push_n(&mut tiles, Tile::Corner90, 10);
        Self::push_n(&mut tiles, Tile::Universal, 4);

        let mut rng = rand::thread_rng();
        tiles.shuffle(&mut rng);

        Self {
            tiles: VecDeque::from(tiles),
            rng,
        }
    }

    fn push_n(tiles: &mut Vec<Tile>, tile: Tile, n: usize) {
        for _ in 0..n {
            tiles.push(tile);
        }
    }

    /// Draw a `Tile` if any remain
    pub fn draw(&mut self) -> Option<Tile> {
        self.tiles.pop_front()
    }

    fn insert_at_random(&mut self, tile: Tile) {
        // Insert at random location
        self.tiles
            // range is exclusive at high end. If there are t tiles, then there
            // are t + 1 insert locations
            .insert(self.rng.gen_range(0, self.tiles.len() + 1), tile);
    }

    /// If a player cannot play, there tiles are returned to the box
    pub fn discard(&mut self, tiles: Vec<Tile>) {
        tiles.into_iter().for_each(|t| self.insert_at_random(t));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn draw_removes_tile_from_box() {
        let mut target = TileBox::new();
        let original_size = target.tiles.len();
        target.draw();
        assert!(target.tiles.len() == original_size - 1);
    }

    #[test]
    fn draw_empty() {
        let mut target = TileBox::new();
        target.tiles = VecDeque::new();
        matches!(target.draw(), None);
    }

    #[test]
    fn discard_inserts() {
        let mut target = TileBox::new();
        target.tiles = VecDeque::from(vec![Tile::Corner90, Tile::Straight]);
        let original_size = target.tiles.len();
        let discarded_tiles = vec![Tile::Universal, Tile::Left135];
        let discarded_size = discarded_tiles.len();
        target.discard(discarded_tiles);
        assert_eq!(target.tiles.len(), original_size + discarded_size);
        assert!(target.tiles.contains(&Tile::Universal));
        assert!(target.tiles.contains(&Tile::Left135));
    }

    #[test]
    fn insert_at_random_when_empty() {
        let mut target = TileBox::new();
        target.tiles = VecDeque::default();
        target.insert_at_random(Tile::Corner90);
        assert_eq!(target.tiles[0], Tile::Corner90)
    }
}

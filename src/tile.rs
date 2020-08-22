use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

/// A unique location on the board
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
            .insert(self.rng.gen_range(0, self.tiles.len()), tile);
    }

    /// If a player cannot play, there tiles are returned to the box
    pub fn discard(&mut self, tiles: Vec<Tile>) {
        tiles.into_iter().for_each(|t| self.insert_at_random(t));
    }
}

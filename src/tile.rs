use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::Add;
use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    SW,
    W,
    NW,
    N,
    NE,
    E,
    SE,
    S,
}

impl Direction {
    pub fn into_offset(self) -> Offset {
        match self {
            Direction::SW => Offset(-1, -1),
            Direction::W => Offset(-1, 0),
            Direction::NW => Offset(-1, 1),
            Direction::N => Offset(0, 1),
            Direction::NE => Offset(1, 1),
            Direction::E => Offset(1, 0),
            Direction::SE => Offset(1, -1),
            Direction::S => Offset(0, -1),
        }
    }
}

#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

#[derive(Copy, Clone, Debug)]
pub struct Offset(i8, i8);

impl Offset {
    pub fn rotate(self, rotation: Rotation) -> Offset {
        let Offset(x, y) = self;
        match rotation {
            Rotation::None => Offset(x, y),
            Rotation::Clockwise90 => Offset(y, -x),
            Rotation::Clockwise180 => Offset(-y, -x),
            Rotation::Clockwise270 => Offset(-y, x),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Coordinates(pub i8, pub i8);

impl Add<Offset> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[wasm_bindgen]
impl Coordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(row: i8, column: i8) -> Self {
        Self(row, column)
    }
}

#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Tile {
    /// ```text
    ///
    /// ---
    ///
    /// ```
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

    pub fn directions(self) -> Vec<Direction> {
        match self {
            Tile::Straight => vec![Direction::S, Direction::N],
            Tile::Diagonal => vec![Direction::SW, Direction::NE],
            Tile::Center90 => vec![Direction::S, Direction::W],
            Tile::Corner90 => vec![Direction::SW, Direction::SE],
            Tile::Left45 => vec![Direction::S, Direction::NW],
            Tile::Right45 => vec![Direction::S, Direction::NE],
            Tile::Left135 => vec![Direction::S, Direction::SW],
            Tile::Right135 => vec![Direction::S, Direction::SE],
            Tile::Universal => vec![
                Direction::S,
                Direction::SW,
                Direction::W,
                Direction::NW,
                Direction::N,
                Direction::NE,
                Direction::E,
                Direction::SE,
            ],
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct TilePlacement {
    pub tile: Tile,
    pub rotation: Rotation,
}

#[derive(Debug)]
pub struct TileBox {
    tiles: VecDeque<Tile>,
    rng: ThreadRng,
}

impl TileBox {
    pub fn new() -> Self {
        // let mut tiles = VecDeque::with_capacity(104);
        let mut tiles = Vec::with_capacity(104);
        // Frequencies from board
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

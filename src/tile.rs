use rand::seq::SliceRandom;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::collections::{VecDeque, HashSet};
use std::hash::Hash;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    SW,
    W,
    NW,
    N,
    NE,
    E,
    SE,
}

impl Direction {
    pub fn path_change(&self, path: Path) -> Path {
        match self {
            Direction::N | Direction::W | Direction::E => path,
            Direction::SW | Direction::NW | Direction::NE | Direction::SE => {
                match path {
                    Path::Diagonal => Path::Orthogonal,
                    Path::Orthogonal => Path::Diagonal,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

#[derive(Copy, Clone, Debug)]
pub enum Path {
    Orthogonal,
    Diagonal,
}

enum NormalTile {

}

#[derive(Copy, Clone, Debug)]
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
    pub fn score(&self) -> i16 {
        match self {
            Tile::Straight | Tile::Diagonal | Tile::Center90 | Tile::Corner90 => 10,
            Tile::Left45 | Tile::Right45 => 8,
            Tile::Left135 | Tile::Right135 => 5,
            Tile::Universal => 35,
        }
    }

    pub fn direction(&self) -> HashSet<Direction> {
        let hash_set = HashSet::new();
        match self {
            Tile::Straight | Tile::Diagonal => {
                hash_set.insert(Direction::N);
            }
            Tile::Center90 => {
                hash_set.insert(Direction::W);
                hash_set.insert(Direction::E);
            }
        };
        hash_set
    }

    pub fn exit(entrance: )
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

    /// If a player cannot play, there tiles are returned to the box
    pub fn discard(&mut self, tile: Tile) {
        // Insert at random location
        self.tiles.insert(self.rng.gen_range(0, self.tiles.len()), tile);
    }
}

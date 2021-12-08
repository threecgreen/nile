use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Eq))]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

pub static ROTATIONS: [Rotation; 4] = [
    Rotation::None,
    Rotation::Clockwise90,
    Rotation::Clockwise180,
    Rotation::Clockwise270,
];

impl Default for Rotation {
    fn default() -> Self {
        Self::None
    }
}

/// A unique location on the board
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinates(pub i8, pub i8);

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row {}, column {}", self.0, self.1)
    }
}

/// A game piece that can be placed on the board
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    /// ```text
    /// ---
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
    ///   /
    ///  /
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
#[derive(Debug, Clone)]
pub struct TileBox {
    tiles: VecDeque<Tile>,
    rng: ThreadRng,
}

impl Default for TileBox {
    fn default() -> Self {
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

        Self::new(tiles)
    }
}

impl TileBox {
    pub fn new(mut tiles: Vec<Tile>) -> Self {
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
        let mut target = TileBox::default();
        let original_size = target.tiles.len();
        target.draw();
        assert!(target.tiles.len() == original_size - 1);
    }

    #[test]
    fn draw_empty() {
        let mut target = TileBox::default();
        target.tiles = VecDeque::new();
        assert!(matches!(target.draw(), None));
    }

    #[test]
    fn discard_inserts() {
        let mut target = TileBox::default();
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
        let mut target = TileBox::default();
        target.tiles = VecDeque::default();
        target.insert_at_random(Tile::Corner90);
        assert_eq!(target.tiles[0], Tile::Corner90)
    }
}

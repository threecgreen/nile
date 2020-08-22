use crate::log::TilePlacementEvent;
use crate::tile::{Coordinates, Rotation, Tile};

use std::ops::{Add, Neg};
use wasm_bindgen::prelude::wasm_bindgen;

/// Similar to `crate::tile::Tile` except without the universal tile, because
/// when placed, a universal tile must represent one of the standard `TilePath`s
#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TilePath {
    Straight,
    Diagonal,
    Center90,
    Corner90,
    Left45,
    Right45,
    Left135,
    Right135,
}

impl TilePath {
    pub fn directions(self) -> [Direction; 2] {
        use TilePath::*;

        match self {
            Straight => [Direction::W, Direction::E],
            Diagonal => [Direction::SW, Direction::NE],
            Center90 => [Direction::S, Direction::W],
            Corner90 => [Direction::SW, Direction::SE],
            Left45 => [Direction::S, Direction::NW],
            Right45 => [Direction::S, Direction::NE],
            Left135 => [Direction::S, Direction::SW],
            Right135 => [Direction::S, Direction::SE],
        }
    }
}

impl From<TilePath> for Tile {
    fn from(tp: TilePath) -> Self {
        match tp {
            TilePath::Straight => Tile::Straight,
            TilePath::Diagonal => Tile::Diagonal,
            TilePath::Center90 => Tile::Center90,
            TilePath::Corner90 => Tile::Corner90,
            TilePath::Left45 => Tile::Left45,
            TilePath::Right45 => Tile::Left45,
            TilePath::Left135 => Tile::Left135,
            TilePath::Right135 => Tile::Left135,
        }
    }
}

pub mod wasm {
    use crate::tile::Tile;

    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    #[derive(Clone, Debug)]
    // wasm_bindgen only supports C-style enums, so we wrap
    // `super::TilePathType` in a tuple struct to make it representable in wasm
    pub struct TilePathType(super::TilePathType);

    #[wasm_bindgen]
    impl TilePathType {
        pub fn universal(tp: super::TilePath) -> Self {
            TilePathType(super::TilePathType::Universal(tp))
        }

        pub fn normal(tp: super::TilePath) -> Self {
            TilePathType(super::TilePathType::Normal(tp))
        }

        pub fn tile_path(&self) -> super::TilePath {
            super::TilePath::from(&self.0)
        }

        pub fn tile(&self) -> Tile {
            Tile::from(&self.0)
        }

        pub fn is_universal(&self) -> bool {
            match self.0 {
                super::TilePathType::Universal(_) => true,
                _ => false,
            }
        }
    }

    impl From<TilePathType> for super::TilePathType {
        fn from(wasm_tpt: TilePathType) -> Self {
            wasm_tpt.0
        }
    }

    impl From<super::TilePathType> for TilePathType {
        fn from(tpt: super::TilePathType) -> Self {
            TilePathType(tpt)
        }
    }
}

#[derive(Clone, Debug)]
pub enum TilePathType {
    Normal(TilePath),
    Universal(TilePath),
}

impl TilePathType {
    pub fn score(&self) -> i16 {
        Tile::from(self).score()
    }

    pub fn directions(&self) -> [Direction; 2] {
        TilePath::from(self).directions()
    }
}

impl From<&TilePathType> for TilePath {
    fn from(tpt: &TilePathType) -> Self {
        match tpt {
            TilePathType::Normal(tp) => *tp,
            TilePathType::Universal(tp) => *tp,
        }
    }
}

impl From<&TilePathType> for Tile {
    fn from(tpt: &TilePathType) -> Self {
        match tpt {
            TilePathType::Normal(tp) => Tile::from(*tp),
            TilePathType::Universal(_) => Tile::Universal,
        }
    }
}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Offset(pub i8, pub i8);

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

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Add<Offset> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn new_offset(
    prev: (Coordinates, Offset),
    placement: &TilePlacementEvent,
) -> Result<Offset, String> {
    // Handle board boundaries elsewhere
    if prev.0 + prev.1 != placement.coordinates {
        return Err("Coordinates dont't align with the rest of the river".to_owned());
    }
    let offsets: Vec<Offset> = placement
        .tile_path_type
        .directions()
        .iter()
        .map(|d| d.into_offset().rotate(placement.rotation))
        .collect();
    let rev_offset = offsets
        .iter()
        .find(|o| placement.coordinates + **o == prev.0)
        .ok_or_else(|| "Tile and rotation don't align with the rest of the river".to_owned())?;
    // TODO: fix for universal
    offsets
        .iter()
        .find(|o| *o != rev_offset)
        .ok_or_else(|| "No valid output offset".to_owned())
        .map(|o| *o)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrong_coordinates() {
        let res = new_offset(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile_path_type: TilePathType::Normal(TilePath::Diagonal),
                rotation: Rotation::Clockwise90,
                coordinates: Coordinates(0, 1),
            },
        );
        matches!(res, Err(msg) if msg.starts_with("Coordinates"));
    }

    #[test]
    fn wrong_tile() {
        let res = new_offset(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile_path_type: TilePathType::Normal(TilePath::Straight),
                rotation: Rotation::None,
                coordinates: Coordinates(1, 1),
            },
        );
        matches!(res, Err(msg) if msg.starts_with("Tile and rotation"));
    }

    #[test]
    fn wrong_rotation() {
        let res = new_offset(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile_path_type: TilePathType::Normal(TilePath::Diagonal),
                rotation: Rotation::Clockwise90,
                coordinates: Coordinates(1, 1),
            },
        );
        matches!(res, Err(msg) if msg.starts_with("Tile and rotation"));
    }

    #[test]
    fn ok_new_offset() {}
}

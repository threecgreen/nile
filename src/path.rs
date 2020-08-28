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
            TilePath::Right45 => Tile::Right45,
            TilePath::Left135 => Tile::Left135,
            TilePath::Right135 => Tile::Right135,
        }
    }
}

pub mod wasm {
    use crate::tile::Tile;

    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn tile_path_to_tile(tile_path: super::TilePath) -> Tile {
        Tile::from(tile_path)
    }

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

        fn tile_path_from_tile(t: Tile) -> Result<super::TilePath, String> {
            match t {
                Tile::Straight => Ok(super::TilePath::Straight),
                Tile::Diagonal => Ok(super::TilePath::Diagonal),
                Tile::Center90 => Ok(super::TilePath::Center90),
                Tile::Corner90 => Ok(super::TilePath::Corner90),
                Tile::Left45 => Ok(super::TilePath::Left45),
                Tile::Right45 => Ok(super::TilePath::Right45),
                Tile::Left135 => Ok(super::TilePath::Left135),
                Tile::Right135 => Ok(super::TilePath::Right135),
                Tile::Universal => Err("Can't convert universal tile to tile path".to_owned()),
            }
        }

        pub fn tile_into_normal(t: Tile) -> Result<TilePathType, JsValue> {
            Ok(Self::normal(Self::tile_path_from_tile(t)?))
        }

        pub fn tile_into_universal(t: Tile) -> Result<TilePathType, JsValue> {
            Ok(Self::universal(Self::tile_path_from_tile(t)?))
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
#[cfg_attr(test, derive(Eq, PartialEq))]
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
            Direction::SW => Offset(1, -1),
            Direction::W => Offset(0, -1),
            Direction::NW => Offset(-1, -1),
            Direction::N => Offset(-1, 0),
            Direction::NE => Offset(-1, 1),
            Direction::E => Offset(0, 1),
            Direction::SE => Offset(1, 1),
            Direction::S => Offset(1, 0),
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
            Rotation::Clockwise180 => Offset(-x, -y),
            Rotation::Clockwise270 => Offset(-y, x),
        }
    }

    pub fn is_diagonal(self) -> bool {
        self.0 != 0 && self.1 != 0
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

/// Given the previous placement, evaluates and validates a single tile
/// placement. The return value of this function can be fed to its next call.
pub fn eval_placement(
    prev: (Coordinates, Offset),
    placement: &TilePlacementEvent,
) -> Result<(Coordinates, Offset), String> {
    // Handle board boundaries elsewhere
    let new_coordinates = prev.0 + prev.1;
    if new_coordinates != placement.coordinates {
        return Err(format!(
            "Tile at {:?} doesn't align with the rest of the river",
            placement.coordinates
        ));
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
        .ok_or_else(|| {
            format!(
                "Tile and rotation at {:?} don't align with the rest of the river",
                placement.coordinates
            )
        })?;
    offsets
        .iter()
        // Assumes there's only two offsets
        .find(|o| *o != rev_offset)
        .ok_or_else(|| format!("No valid output offset for {:?}", placement.coordinates))
        .map(|o| (new_coordinates, *o))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrong_coordinates() {
        let res = eval_placement(
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
        let res = eval_placement(
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
        let res = eval_placement(
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
    fn ok_new_offset() {
        let res = eval_placement(
            (Coordinates(10, 0), Offset(-1, 1)),
            &TilePlacementEvent {
                tile_path_type: TilePathType::Normal(TilePath::Left135),
                rotation: Rotation::None,
                coordinates: Coordinates(9, 1),
            },
        );
        matches!(res, Ok((Coordinates(9, 1), Offset(1, 0))));
    }

    #[test]
    fn left45_second_play() {
        let res = eval_placement(
            (Coordinates(10, 0), Offset(1, 0)),
            &TilePlacementEvent {
                tile_path_type: TilePathType::Normal(TilePath::Left45),
                coordinates: Coordinates(11, 0),
                rotation: Rotation::Clockwise180,
            },
        );
        matches!(res, Ok((Coordinates(11, 0), Offset(1, 1))));
    }

    #[test]
    fn left45_offset() {
        let offsets: Vec<Offset> = TilePath::Left45
            .directions()
            .iter()
            .map(|d| d.into_offset().rotate(Rotation::Clockwise180))
            .collect();
        assert_eq!(offsets, vec![Offset(-1, 0), Offset(1, 1)]);
    }
}

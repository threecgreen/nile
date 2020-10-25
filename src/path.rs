use crate::board::TilePlacement;
use crate::log::TilePlacementEvent;
use crate::tile::{Coordinates, Rotation, Tile, ROTATIONS};

use std::convert::TryFrom;
use std::ops::{Add, Neg};
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents the paths the river can take in a single cell. Similar to
/// `crate::tile::Tile` except without the universal tile, because when placed,
/// a universal tile must represent one of the standard `TilePath`s
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

pub static TILE_PATHS: [TilePath; 8] = [
    TilePath::Straight,
    TilePath::Diagonal,
    TilePath::Center90,
    TilePath::Corner90,
    TilePath::Left45,
    TilePath::Right45,
    TilePath::Left135,
    TilePath::Right135,
];

impl TilePath {
    pub fn offsets(self) -> [Offset; 2] {
        // Representations of the valid offsets as the cardinal directions making
        // the `TilePath` to `Offset` mappings easier to read
        const SW: Offset = Offset(1, -1);
        const W: Offset = Offset(0, -1);
        const NW: Offset = Offset(-1, -1);
        const NE: Offset = Offset(-1, 1);
        const E: Offset = Offset(0, 1);
        const SE: Offset = Offset(1, 1);
        const S: Offset = Offset(1, 0);

        match self {
            TilePath::Straight => [W, E],
            TilePath::Diagonal => [SW, NE],
            TilePath::Center90 => [S, W],
            TilePath::Corner90 => [SW, SE],
            TilePath::Left45 => [S, NW],
            TilePath::Right45 => [S, NE],
            TilePath::Left135 => [S, SW],
            TilePath::Right135 => [S, SE],
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

impl From<TilePathType> for Tile {
    fn from(tpt: TilePathType) -> Self {
        match tpt {
            TilePathType::Normal(tp) => Tile::from(tp),
            TilePathType::Universal(_) => Tile::Universal,
        }
    }
}

impl TryFrom<Tile> for TilePath {
    type Error = String;

    fn try_from(tile: Tile) -> Result<Self, Self::Error> {
        match tile {
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
}

pub mod wasm {
    use crate::tile::Tile;

    use std::convert::TryFrom;
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

        pub fn tile_into_normal(t: Tile) -> Result<TilePathType, JsValue> {
            Ok(Self::normal(super::TilePath::try_from(t)?))
        }

        pub fn tile_into_universal(t: Tile) -> Result<TilePathType, JsValue> {
            Ok(Self::universal(super::TilePath::try_from(t)?))
        }

        pub fn tile_path(&self) -> super::TilePath {
            super::TilePath::from(&self.0)
        }

        pub fn tile(&self) -> Tile {
            Tile::from(&self.0)
        }

        pub fn is_universal(&self) -> bool {
            matches!(self.0, super::TilePathType::Universal(_))
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

/// When a universal tile is placed, the player decides which tile path they
/// would like it to represent. For display purposes and because the player can
/// change which path it represent, it is necessary to differentiate between,
/// for example, a straight tile and a universal tile used as a straight path
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

    pub fn offsets(&self) -> [Offset; 2] {
        TilePath::from(self).offsets()
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

/// An offset on the game board's grid. Used for representing how different
/// tiles can "connect" to form the river.
///
/// Both items of tuple must be of {1, 0, -1}. `Offset(0, 0)` is invalid.
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

pub static OFFSETS: [Offset; 8] = [
    Offset(-1, 1),
    Offset(0, 1),
    Offset(1, 1),
    Offset(1, 0),
    Offset(1, -1),
    Offset(0, -1),
    Offset(-1, -1),
    Offset(-1, 0),
];

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
        .offsets()
        .iter()
        .map(|o| o.rotate(placement.rotation))
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

/// Returns an `Option`, but it should only return `None` in cases where the set
/// of offsets are invalid, e.g. `Offset(1, 0)` and `Offset(-1, 0)`. This set of
/// offsets is invalid because the river would cross over itself.
// TODO: this could probably be memoized, there's 8 x 8 unique offset pairs. Need to
//       measure first.
pub fn offsets_to_tile_placement(prev_offset: Offset, new_offset: Offset) -> Option<TilePlacement> {
    TILE_PATHS.iter().find_map(|tp| {
        ROTATIONS.iter().find_map(|r| {
            let offsets: Vec<_> = tp.offsets().iter().map(|o| o.rotate(*r)).collect();
            // `prev_offset` is flipped because the placement we're looking for is on the
            // "receiving" end of the previous tile's offset
            if (offsets[0] == -prev_offset && offsets[1] == new_offset)
                || (offsets[0] == new_offset && offsets[1] == -prev_offset)
            {
                Some(TilePlacement::new(TilePathType::Normal(*tp), *r))
            } else {
                None
            }
        })
    })
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
        assert!(matches!(res, Err(msg) if msg.contains("doesn't align")));
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
        assert!(matches!(res, Err(msg) if msg.starts_with("Tile and rotation")));
    }

    #[test]
    fn wrong_rotation() {
        let res = eval_placement(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile_path_type: TilePathType::Normal(TilePath::Diagonal),
                rotation: Rotation::None,
                coordinates: Coordinates(1, 1),
            },
        );
        assert!(matches!(res, Err(msg) if msg.starts_with("Tile and rotation")));
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
        assert!(matches!(res, Ok((Coordinates(9, 1), Offset(1, 0)))));
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
        assert!(matches!(res, Ok((Coordinates(11, 0), Offset(1, 1)))));
    }

    #[test]
    fn left45_offset() {
        let offsets: Vec<Offset> = TilePath::Left45
            .offsets()
            .iter()
            .map(|o| o.rotate(Rotation::Clockwise180))
            .collect();
        assert_eq!(offsets, vec![Offset(-1, 0), Offset(1, 1)]);
    }

    #[test]
    fn offsets_to_tile_placement_none() {
        assert!(matches!(
            offsets_to_tile_placement(Offset(1, 0), Offset(-1, 0)),
            None
        ));
    }

    #[test]
    fn offsets_to_tile_placement_some() {
        let tp = offsets_to_tile_placement(Offset(1, 1), Offset(-1, 0)).unwrap();
        assert_eq!(
            tp,
            TilePlacement::new(
                TilePathType::Normal(TilePath::Right135),
                Rotation::Clockwise180
            )
        );

        let tp = offsets_to_tile_placement(Offset(0, -1), Offset(1, 0)).unwrap();
        assert_eq!(
            tp,
            TilePlacement::new(
                TilePathType::Normal(TilePath::Center90),
                Rotation::Clockwise270
            )
        );
    }
}

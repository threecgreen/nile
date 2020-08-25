use crate::log::TilePlacementEvent;
use crate::path::{self, eval_placement, Offset, TilePath, TilePathType};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation};

use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct TilePlacement {
    tile_path_type: TilePathType,
    rotation: Rotation,
}

#[wasm_bindgen]
impl TilePlacement {
    pub fn get_tile_path_type(&self) -> path::wasm::TilePathType {
        path::wasm::TilePathType::from(self.tile_path_type.clone())
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }
}

/// Accessor methods for rust. The fields can't be public because
/// `TilePlacementType` isn't representable in wasm
impl TilePlacement {
    pub fn new(tile_path_type: TilePathType, rotation: Rotation) -> Self {
        Self {
            tile_path_type,
            rotation,
        }
    }

    pub fn tile_path_type(&self) -> &TilePathType {
        &self.tile_path_type
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }

    pub fn offsets(&self) -> Vec<Offset> {
        self.tile_path_type
            .directions()
            .iter()
            .map(|d| d.into_offset().rotate(self.rotation))
            .collect()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Cell {
    bonus: i16,
    tile: Option<TilePlacement>,
}

#[wasm_bindgen]
impl Cell {
    pub fn tile(&self) -> Option<TilePlacement> {
        self.tile.clone()
    }

    pub fn bonus(&self) -> i16 {
        self.bonus
    }
}

impl Cell {
    pub fn with_bonus(bonus: i16) -> Self {
        Self { bonus, tile: None }
    }

    pub fn set_tile(&mut self, tile: TilePlacement) -> TurnScore {
        self.tile = Some(tile);
        self.score()
    }

    pub fn remove_tile(&mut self) -> Option<(TilePlacement, TurnScore)> {
        let old_score = self.score();
        self.tile.take().map(|tp| (tp, -old_score))
    }

    pub fn is_empty(&self) -> bool {
        self.tile.is_none()
    }

    pub fn score(&self) -> TurnScore {
        let tile_score = self
            .tile
            .as_ref()
            .map(|t| t.tile_path_type.score())
            .unwrap_or(0);
        TurnScore::from(tile_score) + TurnScore::from(self.bonus)
    }

    pub fn update_universal_path(&mut self, tile_path: TilePath) -> Result<TilePath, String> {
        let tile_placement = self
            .tile
            .as_mut()
            .ok_or_else(|| "Cell is empty".to_owned())?;
        let old_tile_path = match tile_placement.tile_path_type {
            TilePathType::Normal(_) => {
                return Err("Cell doesn't contain a universal tile".to_owned());
            }
            TilePathType::Universal(tp) => tp,
        };
        tile_placement.tile_path_type = TilePathType::Universal(tile_path);

        Ok(old_tile_path)
    }
}

/// The board is 21x21 plus a special end of game column
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Board {
    last_placement: (Coordinates, Offset),
    cells: Vec<Cell>,
    end_of_game_cells: Vec<Cell>,
}

static BOARD_SIZE: usize = 21;

macro_rules! hash_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

impl Board {
    pub fn new() -> Board {
        let bonus_order: Vec<i16> = vec![50, 50, 50, 50, 75, 75, 100, 100, 150, 200, 500];
        let bonuses: HashMap<(usize, usize), i16> = hash_map!(
            (1, 6) => -80,
            (1, 10) => -100,
            (1, 14) => -120,
            (2, 2) => -40,
            (2, 3) => -40,
            (2, 17) => -160,
            (2, 18) => -160,
            (3, 2) => -40,
            (3, 8) => 140,
            (3, 12) => 220,
            (3, 18) => -160,
            (4, 2) => -40,
            (4, 3) => -40,
            (4, 17) => -160,
            (4, 18) => -160,
            (5, 6) => -80,
            (5, 10) => -100,
            (5, 14) => -120,
            (7, 2) => 60,
            (7, 10) => 240,
            (7, 18) => 250,
            (8, 7) => 240,
            (8, 13) => 240,
            (9, 5) => -60,
            (9, 9) => -40,
            (9, 11) => -40,
            (9, 15) => -140,
            (10, 10) => -60,
            (10, 18) => -160
        );
        let cells: Vec<Cell> = (0..BOARD_SIZE * BOARD_SIZE)
            .map(|i| {
                let row = {
                    let row = i / BOARD_SIZE;
                    // Board is reflected across horizontal axis
                    if row > 10 {
                        BOARD_SIZE - 1 - row
                    } else {
                        row
                    }
                };
                let col = i % BOARD_SIZE;
                bonuses
                    .get(&(row, col))
                    .map(|b| Cell::with_bonus(*b))
                    .unwrap_or_default()
            })
            .collect();
        Self {
            // Start arrow placement and offset
            last_placement: (Coordinates(10, -1), Offset(0, 1)),
            cells,
            // Symmetrical
            end_of_game_cells: bonus_order
                .iter()
                .chain(bonus_order.iter().rev().skip(1))
                .map(|b| Cell::with_bonus(*b))
                .collect(),
        }
    }

    pub fn cell(&self, coordinates: Coordinates) -> Option<&Cell> {
        let width = self.width();
        if coordinates.1 as usize == self.width() {
            self.end_of_game_cells.get(coordinates.0 as usize)
        } else {
            let row = coordinates.0 as usize;
            let column = coordinates.1 as usize;
            self.cells.get(row * width + column)
        }
    }

    fn get_mut_cell(&mut self, coordinates: Coordinates) -> Option<&mut Cell> {
        let width = self.width();
        if coordinates.1 as usize == self.width() {
            self.end_of_game_cells.get_mut(coordinates.0 as usize)
        } else {
            let row = coordinates.0 as usize;
            let column = coordinates.1 as usize;
            self.cells.get_mut(row * width + column)
        }
    }

    pub fn place_tile(
        &mut self,
        coordinates: Coordinates,
        tile_placement: TilePlacement,
    ) -> Result<TurnScore, String> {
        match self.get_mut_cell(coordinates) {
            Some(cell) if cell.is_empty() => Ok(cell.set_tile(tile_placement)),
            Some(_) => Err("There's already a tile there".to_owned()),
            None => Err("Invalid coordinates".to_owned()),
        }
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<(), String> {
        let cell = self
            .get_mut_cell(coordinates)
            .ok_or_else(|| "Invalid coordinates".to_owned())?;
        if let Some(ref mut tile) = cell.tile {
            tile.rotation = rotation;
            Ok(())
        } else {
            Err("Cell is empty".to_owned())
        }
    }

    pub fn remove_tile(&mut self, coordinates: Coordinates) -> Option<(TilePlacement, TurnScore)> {
        match self.get_mut_cell(coordinates) {
            Some(cell) => cell.remove_tile(),
            None => None,
        }
    }

    /// Returns the old `TilePath`
    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        tile_path: TilePath,
    ) -> Result<TilePath, String> {
        match self.get_mut_cell(coordinates) {
            Some(cell) => cell.update_universal_path(tile_path),
            None => Err("Invalid coordinates".to_owned()),
        }
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> Result<TurnScore, String> {
        let (tile_placement, removal_score) = self
            .remove_tile(old_coordinates)
            .ok_or_else(|| "Can't move tile when there's no tile at old coordinates".to_owned())?;
        let placement_score = self
            .place_tile(new_coordinates, tile_placement.clone())
            .map_err(|e| {
                // Try to replace the tile
                self.place_tile(old_coordinates, tile_placement)
                    .expect("should be able to replace tile");
                e
            })?;
        // `removal_score` should already be negative to revert the score effect of placement
        Ok(placement_score + removal_score)
    }

    pub fn last_placement(&self) -> (Coordinates, Offset) {
        self.last_placement
    }

    pub fn has_tile(&self, coordinates: Coordinates) -> bool {
        self.cell(coordinates)
            .map(|cell| !cell.is_empty())
            .unwrap_or_default()
    }

    /// Called as part of end of turn
    pub fn validate_turns_moves(
        &mut self,
        mut turn_coordinates: HashSet<Coordinates>,
    ) -> Result<(), String> {
        let mut last_placement = self.last_placement;
        while !turn_coordinates.is_empty() {
            let coordinates = last_placement.0 + last_placement.1;
            let cell = self
                .cell(coordinates)
                .ok_or_else(|| format!("Invalid coordinates {:?}", coordinates))?;
            let tile = cell
                .tile()
                .ok_or_else(|| format!("Non-contiguous path. No tile at {:?}", coordinates))?;
            last_placement = eval_placement(
                last_placement,
                &TilePlacementEvent {
                    tile_path_type: tile.tile_path_type,
                    rotation: tile.rotation,
                    coordinates,
                },
            )?;
            self.no_crossover(last_placement.0, last_placement.1)?;
            if !turn_coordinates.remove(&last_placement.0) {
                return Err("Can't reuse a tile from another turn".to_owned());
            }
        }
        // Check last tile doesn't end in another tile
        if self.has_tile(last_placement.0 + last_placement.1) {
            return Err(format!(
                "Can't play a tile at {:?} because it dead-ends into the rest of the river at {:?}",
                last_placement.0 + last_placement.1,
                last_placement.0,
            ));
        }
        self.last_placement = last_placement;
        Ok(())
    }

    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        // FIXME: update for end of game
        (0..self.width() as i8).contains(&coordinates.0)
            && (0..self.height() as i8).contains(&coordinates.1)
    }

    /// Checks whether a placement would result in a crossover. This is invalid
    /// and can only happen with diagonal offsets. Orthogonal offsets would collide
    /// with another tile and that's handled by other checks.
    ///
    /// A path like the drawing below is invalid.
    /// ```text
    ///  |\ /
    ///  | X
    ///  |/ \
    /// ```
    pub fn no_crossover(&self, coordinates: Coordinates, offset: Offset) -> Result<(), String> {
        if !offset.is_diagonal() {
            return Ok(());
        }
        let coordinates1 = coordinates + Offset(offset.0, 0);
        let coordinates2 = coordinates + Offset(0, offset.1);
        if let (Some(tp1), Some(tp2)) = (
            self.cell(coordinates1).and_then(|c| c.tile.as_ref()),
            self.cell(coordinates2).and_then(|c| c.tile.as_ref()),
        ) {
            if tp1
                .offsets()
                .into_iter()
                .any(|o| coordinates1 + o == coordinates2)
                && tp2
                    .offsets()
                    .into_iter()
                    .any(|o| coordinates2 + o == coordinates1)
            {
                return Err(format!(
                    "The river cannot cross over existing path between {:?} and {:?}. Invalid tile placement at {:?}",
                    coordinates1,
                    coordinates2,
                    coordinates
                ));
            }
        }
        Ok(())
    }

    // /// Determines if river is completely encircled and there is not 'escape'.
    // /// This incidates one or more moves are invalid
    // pub fn no_encircles(&self) -> Result<(), String> {
    //     Ok(())
    // }
}

pub mod wasm {
    use super::{Board, Cell, Coordinates, BOARD_SIZE};

    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Board {
        pub fn height(&self) -> usize {
            BOARD_SIZE
        }

        pub fn width(&self) -> usize {
            BOARD_SIZE
        }

        pub fn get_cell(&self, row: i8, column: i8) -> Result<Cell, JsValue> {
            self.cell(Coordinates(row, column))
                .cloned()
                .ok_or_else(|| JsValue::from("Invalid coordinates"))
        }
    }
}

#[cfg(test)]
impl Board {
    pub fn with_last_placement(coordinates: Coordinates, offset: Offset) -> Self {
        let mut board = Self::new();
        board.last_placement = (coordinates, offset);
        board
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::FromIterator;

    #[test]
    fn set_and_remove_are_idempotent() {
        let mut target = Cell::with_bonus(0);
        let original_score = target.score();
        assert!(target.is_empty());
        assert_eq!(original_score.score(), 0);
        let tile_placement = TilePlacement {
            rotation: Rotation::None,
            tile_path_type: TilePathType::Normal(TilePath::Straight),
        };
        let placement_score = target.set_tile(tile_placement.clone());
        assert!(!target.is_empty());
        assert!(placement_score.score() > original_score.score());
        let (removed_tile_placement, opp_score) = target.remove_tile().unwrap();
        assert_eq!(target.score().score(), 0);
        assert!(target.is_empty());
        assert_eq!(opp_score, -placement_score);
        assert_eq!(removed_tile_placement, tile_placement);
    }

    #[test]
    fn score_includes_bonus() {
        let target = Cell {
            tile: Some(TilePlacement {
                rotation: Rotation::Clockwise270,
                tile_path_type: TilePathType::Universal(TilePath::Diagonal),
            }),
            bonus: 80,
        };
        assert_eq!(target.score().score(), 115);
    }

    #[test]
    fn score_includes_penalty() {
        let bonus = -80;
        let mut target = Cell::with_bonus(bonus);
        assert_eq!(target.score().score(), bonus);
        target.set_tile(TilePlacement {
            rotation: Rotation::None,
            tile_path_type: TilePathType::Normal(TilePath::Right45),
        });
        assert_eq!(target.score().score(), bonus + 8);
    }

    #[test]
    fn update_universal_path_on_cell_fails_for_empty() {
        let mut target = Cell::with_bonus(0);
        let res = target.update_universal_path(TilePath::Diagonal);
        matches!(res, Err(e) if e.contains("empty"));
    }

    #[test]
    fn update_universal_path_on_cell_fails_for_normal_tile() {
        let mut target = Cell {
            tile: Some(TilePlacement {
                rotation: Rotation::Clockwise90,
                tile_path_type: TilePathType::Normal(TilePath::Left45),
            }),
            bonus: 0,
        };
        let res = target.update_universal_path(TilePath::Right45);
        matches!(res, Err(e) if e.contains("doesn't contain a universal tile"));
    }

    #[test]
    fn update_universal_path_on_cell() {
        let mut target = Cell {
            tile: Some(TilePlacement {
                rotation: Rotation::Clockwise90,
                tile_path_type: TilePathType::Universal(TilePath::Left45),
            }),
            bonus: 0,
        };
        let res = target.update_universal_path(TilePath::Right45);
        matches!(res, Ok(TilePath::Left45));
    }

    #[test]
    fn board_cell_works_with_end_game() {
        let target = Board::new();
        let end_game_cell = target.cell(Coordinates(10, 21));
        matches!(end_game_cell, Some(cell) if cell.bonus() == 500);
    }

    #[test]
    fn first_turn_validation() {
        let mut target = Board::new();
        let coordinates = Coordinates(10, 0);
        target
            .place_tile(
                coordinates,
                TilePlacement {
                    rotation: Rotation::None,
                    tile_path_type: TilePathType::Normal(TilePath::Straight),
                },
            )
            .unwrap();
        let mut coordinates_set = HashSet::new();
        coordinates_set.insert(coordinates);
        let res = target.validate_turns_moves(coordinates_set);
        assert!(res.is_ok());
    }

    #[test]
    fn validate_45() {
        let mut target = Board::new();
        target
            .place_tile(
                Coordinates(10, 0),
                TilePlacement::new(TilePathType::Normal(TilePath::Center90), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(11, 0),
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Left45),
                    Rotation::Clockwise180,
                ),
            )
            .unwrap();
        let mut coordinates_set = HashSet::new();
        coordinates_set.insert(Coordinates(10, 0));
        coordinates_set.insert(Coordinates(11, 0));
        let res = target.validate_turns_moves(coordinates_set);
        assert!(res.is_ok());
    }

    #[test]
    fn no_crossover() {
        let mut target = Board::new();
        let coordinates = vec![Coordinates(5, 1), Coordinates(5, 0), Coordinates(6, 0)];
        target
            .place_tile(
                coordinates[0],
                TilePlacement::new(TilePathType::Normal(TilePath::Diagonal), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[1],
                TilePlacement::new(TilePathType::Normal(TilePath::Right135), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[2],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Left135),
                    Rotation::Clockwise180,
                ),
            )
            .unwrap();
        let coordinates_set = HashSet::from_iter(coordinates.iter().cloned());
        let res = target.validate_turns_moves(coordinates_set);
        matches!(res, Err(msg) if msg.contains("cross over"));
    }

    #[test]
    fn universal_start() {
        let mut target = Board::new();
        let coordinates = vec![
            Coordinates(10, 0),
            Coordinates(11, 1),
            Coordinates(12, 2),
            Coordinates(13, 3),
            Coordinates(12, 4),
        ];
        target
            .place_tile(
                coordinates[0],
                TilePlacement::new(
                    TilePathType::Universal(TilePath::Right45),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[1],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Diagonal),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[2],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Diagonal),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[3],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Corner90),
                    Rotation::Clockwise180,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[4],
                TilePlacement::new(TilePathType::Normal(TilePath::Left135), Rotation::None),
            )
            .unwrap();
        let coordinates_set = HashSet::from_iter(coordinates.iter().cloned());
        let res = target.validate_turns_moves(coordinates_set);
        matches!(res, Ok(()));
    }

    #[test]
    fn crossover_is_invalid() {
        let mut target = Board::new();
        let coordinates = vec![
            Coordinates(14, 5),
            Coordinates(14, 4),
            Coordinates(14, 3),
            Coordinates(13, 2),
            Coordinates(13, 3),
        ];
        target
            .place_tile(
                coordinates[0],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Left135),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[1],
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[2],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Right45),
                    Rotation::Clockwise270,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[3],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Left135),
                    Rotation::Clockwise270,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[4],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Center90),
                    Rotation::Clockwise180,
                ),
            )
            .unwrap();
        let coordinates_set = HashSet::from_iter(coordinates.iter().cloned());
        let res = target.validate_turns_moves(coordinates_set);
        matches!(res, Err(_));
    }

    #[test]
    fn end_with_dead_end() {
        let mut target = Board::new();
        let coordinates = vec![
            Coordinates(6, 2),
            Coordinates(7, 3),
            Coordinates(7, 2),
            Coordinates(7, 1),
        ];
        target
            .place_tile(
                coordinates[0],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Diagonal),
                    Rotation::Clockwise270,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[1],
                TilePlacement::new(
                    TilePathType::Universal(TilePath::Left135),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[2],
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[3],
                TilePlacement::new(
                    TilePathType::Universal(TilePath::Right135),
                    Rotation::Clockwise270,
                ),
            )
            .unwrap();
        let coordinates_set = HashSet::from_iter(coordinates.iter().cloned());
        let res = target.validate_turns_moves(coordinates_set);
        matches!(res, Err(_));
    }

    #[test]
    fn has_tile() {
        let mut target = Board::new();
        let coordinates = Coordinates(10, 3);
        assert!(!target.has_tile(coordinates));
        target
            .place_tile(
                coordinates,
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        assert!(target.has_tile(coordinates));
    }
}

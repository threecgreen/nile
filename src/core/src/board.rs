use crate::error::{self, CellError, Error};
use crate::log::TilePlacementEvent;
use crate::path::{self, eval_placement, Offset, TilePath, TilePathType};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation};

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct TilePlacement {
    tile_path_type: TilePathType,
    rotation: Rotation,
}

/// Accessor methods for rust.
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
            .offsets()
            .iter()
            .map(|o| o.rotate(self.rotation))
            .collect()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Cell {
    bonus: i16,
    tile: Option<TilePlacement>,
}

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
                return Err("Cell doesn’t contain a universal tile".to_owned());
            }
            TilePathType::Universal(tp) => tp,
        };
        tile_placement.tile_path_type = TilePathType::Universal(tile_path);

        Ok(old_tile_path)
    }
}

/// The board is 21x21 plus a special end of game column
#[derive(Clone, Debug)]
pub struct Board {
    last_placement: (Coordinates, Offset),
    cells: Vec<Cell>,
    end_of_game_cells: Vec<Cell>,
}

pub const BOARD_DIM: usize = 21;

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

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

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
        let cells: Vec<Cell> = (0..BOARD_DIM * BOARD_DIM)
            .map(|i| {
                let row = {
                    let row = i / BOARD_DIM;
                    // Board is reflected across horizontal axis
                    if row > 10 {
                        BOARD_DIM - 1 - row
                    } else {
                        row
                    }
                };
                let col = i % BOARD_DIM;
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
        let width = BOARD_DIM;
        if self.is_end_game_cell(coordinates) {
            self.end_of_game_cells.get(coordinates.0 as usize)
        } else if self.in_bounds(coordinates) {
            let row = coordinates.0 as usize;
            let column = coordinates.1 as usize;
            self.cells.get(row * width + column)
        } else {
            None
        }
    }

    fn get_mut_cell(&mut self, coordinates: Coordinates) -> Option<&mut Cell> {
        let width = BOARD_DIM;
        if coordinates.1 as usize == BOARD_DIM {
            self.end_of_game_cells.get_mut(coordinates.0 as usize)
        } else if self.in_bounds(coordinates) {
            let row = coordinates.0 as usize;
            let column = coordinates.1 as usize;
            self.cells.get_mut(row * width + column)
        } else {
            None
        }
    }

    pub fn place_tile(
        &mut self,
        coordinates: Coordinates,
        tile_placement: TilePlacement,
    ) -> Result<TurnScore, CellError> {
        match self.get_mut_cell(coordinates) {
            Some(cell) if cell.is_empty() => Ok(cell.set_tile(tile_placement)),
            Some(_) => Err(CellError::new(
                coordinates,
                "There’s already a tile there".to_owned(),
            )),
            None => Err(CellError::new(
                coordinates,
                "Invalid coordinates".to_owned(),
            )),
        }
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        rotation: Rotation,
    ) -> Result<(), CellError> {
        let cell = self
            .get_mut_cell(coordinates)
            .ok_or_else(|| CellError::new(coordinates, "Invalid coordinates".to_owned()))?;
        if let Some(ref mut tile) = cell.tile {
            tile.rotation = rotation;
            Ok(())
        } else {
            Err(CellError::new(coordinates, "Cell is empty".to_owned()))
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
    ) -> error::Result<TilePath> {
        match self.get_mut_cell(coordinates) {
            Some(cell) => cell
                .update_universal_path(tile_path)
                .map_err(|msg| Error::cell(coordinates, msg)),
            None => Err(Error::cell(coordinates, "Invalid coordinates".to_owned())),
        }
    }

    pub fn move_tile(
        &mut self,
        old_coordinates: Coordinates,
        new_coordinates: Coordinates,
    ) -> Result<TurnScore, CellError> {
        let (tile_placement, removal_score) =
            self.remove_tile(old_coordinates).ok_or_else(|| {
                CellError::new(
                    old_coordinates,
                    "Can’t move tile when there’s no tile at old coordinates".to_owned(),
                )
            })?;
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

    pub fn is_end_game_cell(&self, coordinates: Coordinates) -> bool {
        let Coordinates(_, column) = coordinates;
        column == BOARD_DIM as i8
    }

    /// Called as part of end of turn. Returns whether the game has ended, i.e.
    /// a tile has been placed in the final column of cells.
    pub fn validate_turns_moves(
        &mut self,
        mut turn_coordinates: HashSet<Coordinates>,
    ) -> crate::error::Result<bool> {
        let mut last_placement = self.last_placement;
        while !turn_coordinates.is_empty() {
            let coordinates = last_placement.0 + last_placement.1;
            let cell = self.cell(coordinates).ok_or_else(|| {
                Error::cell(coordinates, format!("Invalid coordinates: {}", coordinates))
            })?;
            let tile = cell.tile().ok_or_else(|| {
                Error::cell(
                    coordinates,
                    format!("Non-contiguous path. Missing tile at {}", coordinates),
                )
            })?;
            last_placement = eval_placement(
                last_placement,
                &TilePlacementEvent {
                    tile_path_type: tile.tile_path_type,
                    rotation: tile.rotation,
                    coordinates,
                },
            )
            .map_err(Error::Cell)?;
            self.no_crossover(last_placement.0, last_placement.1)?;
            if !turn_coordinates.remove(&last_placement.0) {
                return Err(Error::cell(
                    last_placement.0,
                    "Can’t reuse a tile from another turn".to_owned(),
                ));
            }
        }
        // Check last tile doesn't end in another tile
        if self.has_tile(last_placement.0 + last_placement.1) {
            let mut err_coordinates = HashSet::new();
            err_coordinates.insert(last_placement.0);
            err_coordinates.insert(last_placement.0 + last_placement.1);
            return Err(Error::cells(
                err_coordinates,
                format!(
                    "Can’t play a tile at {} because it dead-ends into the rest of the river at {}",
                    last_placement.0,
                    last_placement.0 + last_placement.1
                ),
            ));
        }
        // Check if multiple tiles in end of game area
        let end_of_game_cell_count = self
            .end_of_game_cells
            .iter()
            .filter(|c| !c.is_empty())
            .count();
        // Check this turns doesn't leave the river encircled
        self.no_encircles(last_placement).map_err(Error::Cell)?;
        let has_ended = Self::validate_end_of_game_cells(end_of_game_cell_count, last_placement)?;
        self.last_placement = last_placement;
        Ok(has_ended)
    }

    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        (0..BOARD_DIM as i8).contains(&coordinates.0)
            // +1 for end of game tile
            && (0..BOARD_DIM as i8 + 1).contains(&coordinates.1)
    }

    pub fn validate_end_of_game_cells(
        end_of_game_cell_count: usize,
        last_placement: (Coordinates, Offset),
    ) -> crate::error::Result<bool> {
        let (Coordinates(_row, column), offset) = last_placement;
        match end_of_game_cell_count {
            1 if offset == Offset(0, 1) && column as usize == BOARD_DIM => Ok(true),
            1 if column as usize == BOARD_DIM => Err(Error::cell(
                last_placement.0,
                format!(
                    "Tile in end-of-game column at {} must align with the dot",
                    last_placement.0
                ),
            )),
            1 => Err(Error::cell(
                last_placement.0,
                "Tile placed in end-of-game column must be the last tile of the river".to_owned(),
            )),
            0 => Ok(false),
            _ => Err(Error::Msg(
                "Can’t play more than one tile in the end-of-game column".to_owned(),
            )),
        }
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
    pub fn no_crossover(
        &self,
        coordinates: Coordinates,
        offset: Offset,
    ) -> crate::error::Result<()> {
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
                let mut err_coordinates = HashSet::new();
                err_coordinates.insert(coordinates);
                err_coordinates.insert(coordinates1);
                err_coordinates.insert(coordinates2);
                return Err(Error::cells(err_coordinates, format!("The river cannot cross over existing path between {} and {}. Invalid tile placement at {}", coordinates1, coordinates2, coordinates)));
            }
        }
        Ok(())
    }

    /// Determines if river is completely encircled and there is not 'escape'.
    /// This incidates one or more moves are invalid
    ///
    /// Parameterizing `turn_coordinates` and `last_placement` allows this
    /// function to be more easily used by the AI in determining which potential
    /// moves are valid. This method is _almost_ static and can easily be tested
    /// with an empty board and all state passed in via the arguments.
    fn open_moves(&self) -> Vec<Offset> {
        let next_coordinates = self.last_placement.0 + self.last_placement.1;
        path::OFFSETS
            .iter()
            .filter_map(|offset| {
                if self.no_crossover(next_coordinates, *offset).is_err() {
                    return None;
                }
                let coordinates = next_coordinates + *offset;
                match self.cell(coordinates) {
                    Some(cell) if cell.is_empty() => Some(*offset),
                    _ => None,
                }
            })
            .collect()
    }

    fn no_encircles(&self, last_placement: (Coordinates, Offset)) -> Result<(), CellError> {
        let (last_coordinates, _) = last_placement;
        if self.is_end_game_cell(last_coordinates) {
            return Ok(());
        }
        let mut visited = HashSet::new();
        self.no_encircles_impl(last_placement, &mut visited)
    }

    fn no_encircles_impl(
        &self,
        last_placement: (Coordinates, Offset),
        visited: &mut HashSet<Coordinates>,
    ) -> Result<(), CellError> {
        let (last_coordinates, last_offset) = last_placement;
        let coordinates = last_coordinates + last_offset;
        if self.is_end_game_cell(coordinates) {
            return Ok(());
        }
        visited.insert(coordinates);
        let mut copy = self.clone();
        copy.last_placement = last_placement;
        copy.cell(coordinates).ok_or_else(|| {
            CellError::new(
                coordinates,
                format!(
                    "Invalid river path leading to coordinates: {} that are off the board",
                    coordinates
                ),
            )
        })?;
        let open_moves = copy.open_moves();

        for offset in open_moves.iter() {
            if visited.contains(&(coordinates + *offset)) {
                continue;
            }
            if let Some(tp) = path::offsets_to_tile_placement(last_offset, *offset) {
                if copy.place_tile(coordinates, tp).is_ok() {
                    match copy.no_encircles_impl((coordinates, *offset), visited) {
                        Ok(()) => return Ok(()),
                        Err(_) => {
                            copy.remove_tile(coordinates);
                        }
                    }
                }
            }
        }
        Err(CellError::new(coordinates, format!("Encircled river path. There are no paths leading to the end of game column from {} with open moves {:?}", coordinates, open_moves)))
    }
}

#[cfg(test)]
impl Board {
    /// Test constructor
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

    macro_rules! hash_set(
        { $($key:expr),+ } => {
            {
                let mut s = ::std::collections::HashSet::new();
                $(
                    s.insert($key);
                )+
                s
            }
        }
    );

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
        assert!(matches!(res, Err(e) if e.contains("empty")));
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
        assert!(matches!(res, Err(e) if e.contains("doesn’t contain a universal tile")));
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
        assert!(matches!(res, Ok(TilePath::Left45)));
    }

    #[test]
    fn board_cell_works_with_end_game() {
        let target = Board::new();
        let end_game_cell = target.cell(Coordinates(10, 21));
        assert!(matches!(end_game_cell, Some(cell) if cell.bonus() == 500));
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
        let mut target = Board::with_last_placement(Coordinates(4, 2), Offset(1, -1));
        let coordinates = vec![Coordinates(5, 1), Coordinates(6, 0), Coordinates(6, 1)];
        target
            .place_tile(
                coordinates[0],
                TilePlacement::new(TilePathType::Normal(TilePath::Diagonal), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[1],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Right135),
                    Rotation::Clockwise270,
                ),
            )
            .unwrap();
        target
            .place_tile(
                coordinates[2],
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Left135),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        let coordinates_set = HashSet::from_iter(coordinates.iter().cloned());
        let res = target.validate_turns_moves(coordinates_set);
        assert!(matches!(res, Err(Error::Cell(err)) if err.msg.contains("cross over")));
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
        assert!(matches!(res, Ok(false)));
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
        assert!(res.is_err());
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
        assert!(res.is_err());
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

    #[test]
    fn is_end_game_cell() {
        let target = Board::new();
        assert!(target.is_end_game_cell(Coordinates(0, 21)));
        assert!(target.is_end_game_cell(Coordinates(21, 21)));
        assert!(!target.is_end_game_cell(Coordinates(21, 0)));
        assert!(!target.is_end_game_cell(Coordinates(0, 0)));
    }

    #[test]
    fn in_bounds() {
        let target = Board::new();
        assert!(target.in_bounds(Coordinates(0, 21)));
        assert!(target.in_bounds(Coordinates(10, 21)));
        assert!(!target.in_bounds(Coordinates(0, 22)));
        assert!(!target.in_bounds(Coordinates(21, 20)));
        assert!(target.in_bounds(Coordinates(20, 10)));
        assert!(target.in_bounds(Coordinates(10, 10)));
    }

    #[test]
    fn cell_out_of_bounds_cell_is_none() {
        let target = Board::new();
        assert!(matches!(target.cell(Coordinates(14, 22)), None));
    }

    #[test]
    fn validate_turns_moves_ends_game() {
        let mut target = Board::with_last_placement(Coordinates(14, 20), Offset(0, 1));
        const COORDINATES: Coordinates = Coordinates(14, BOARD_DIM as i8);
        target
            .place_tile(
                COORDINATES,
                TilePlacement {
                    rotation: Rotation::None,
                    tile_path_type: TilePathType::Normal(TilePath::Straight),
                },
            )
            .unwrap();
        let turn_coordinates = hash_set!(COORDINATES);
        let res = target.validate_turns_moves(turn_coordinates);
        assert!(matches!(res, Ok(true)));
    }

    fn setup_encircled_board() -> Board {
        // Test same board set up with different last offsets
        let mut target = Board::new();

        for i in 0..3 {
            target
                .place_tile(
                    Coordinates(3 - i, 0),
                    TilePlacement {
                        rotation: Rotation::Clockwise90,
                        tile_path_type: TilePathType::Normal(TilePath::Straight),
                    },
                )
                .unwrap();
        }
        target
            .place_tile(
                Coordinates(0, 0),
                TilePlacement {
                    rotation: Rotation::Clockwise270,
                    tile_path_type: TilePathType::Normal(TilePath::Center90),
                },
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(0, 1),
                TilePlacement {
                    rotation: Rotation::None,
                    tile_path_type: TilePathType::Normal(TilePath::Straight),
                },
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(0, 2),
                TilePlacement {
                    rotation: Rotation::Clockwise90,
                    tile_path_type: TilePathType::Normal(TilePath::Right45),
                },
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(1, 3),
                TilePlacement {
                    rotation: Rotation::None,
                    tile_path_type: TilePathType::Normal(TilePath::Left45),
                },
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(2, 3),
                TilePlacement {
                    rotation: Rotation::Clockwise90,
                    tile_path_type: TilePathType::Normal(TilePath::Straight),
                },
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(3, 3),
                TilePlacement {
                    rotation: Rotation::Clockwise90,
                    tile_path_type: TilePathType::Normal(TilePath::Center90),
                },
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(3, 2),
                TilePlacement {
                    rotation: Rotation::Clockwise270,
                    tile_path_type: TilePathType::Normal(TilePath::Right45),
                },
            )
            .unwrap();
        target
    }

    /// Subset of board for testing
    /// ```text
    ///   0 1 2 3
    /// 0 +-+-+ .
    ///   |    \
    /// 1 + . . +
    ///   |     |
    /// 2 + + . +
    ///   |  \  |
    /// 3 + . +-+
    ///   ^
    ///   |
    /// start
    /// ```
    #[test]
    fn no_encircles_depends_on_offset() {
        let mut target = setup_encircled_board();
        target
            .place_tile(
                Coordinates(2, 1),
                TilePlacement {
                    rotation: Rotation::Clockwise270,
                    tile_path_type: TilePathType::Normal(TilePath::Diagonal),
                },
            )
            .unwrap();
        let res = target.no_encircles((Coordinates(2, 1), Offset(-1, -1)));
        assert!(res.is_err());

        target = setup_encircled_board();
        target
            .place_tile(
                Coordinates(2, 1),
                TilePlacement {
                    rotation: Rotation::Clockwise180,
                    tile_path_type: TilePathType::Normal(TilePath::Left45),
                },
            )
            .unwrap();
        let res = target.no_encircles((Coordinates(2, 1), Offset(-1, 0)));
        assert!(res.is_err());

        target = setup_encircled_board();
        target
            .place_tile(
                Coordinates(2, 1),
                TilePlacement::new(TilePathType::Normal(TilePath::Right135), Rotation::None),
            )
            .unwrap();
        let res = target.no_encircles((Coordinates(2, 1), Offset(1, 0)));
        assert!(res.is_ok());
    }

    #[test]
    fn no_encircle_valid_start() {
        let mut target = Board::new();
        target
            .place_tile(
                Coordinates(10, 0),
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Right45),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(11, 1),
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Diagonal),
                    Rotation::Clockwise180,
                ),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(12, 2),
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Corner90),
                    Rotation::Clockwise90,
                ),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(13, 1),
                TilePlacement::new(
                    TilePathType::Normal(TilePath::Left135),
                    Rotation::Clockwise180,
                ),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(12, 1),
                TilePlacement::new(TilePathType::Normal(TilePath::Center90), Rotation::None),
            )
            .unwrap();
        let res = target.no_encircles((Coordinates(12, 1), Offset(0, -1)));
        assert!(res.is_ok());
    }

    #[test]
    fn no_encircle_doesnt_prevent_game_end() {
        let mut target = Board::new();
        target
            .place_tile(
                Coordinates(12, 19),
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(12, 20),
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        target
            .place_tile(
                Coordinates(12, 21),
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        let res = target.no_encircles((Coordinates(12, 21), Offset(0, 1)));
        assert!(res.is_ok());
    }
}

use super::CPUPlayer;
use crate::board::Board;
use crate::log::{Event, TilePlacementEvent};
use crate::path::{eval_placement, Offset, TilePath, TilePathType};
use crate::score::TurnScore;
use crate::tile::{Coordinates, Rotation, Tile};

use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Brute {}

impl CPUPlayer for Brute {
    fn take_turn(&mut self, tiles: &VecDeque<Tile>, board: &Board) -> Vec<Event> {
        let last_placement = board.last_placement();
        match Self::best_moves(
            board,
            last_placement.0,
            last_placement.1,
            TurnScore::default(),
            tiles,
            &Vec::new(),
        ) {
            Some(moves) => {
                let mut events: Vec<Event> =
                    moves.placements.into_iter().map(Event::PlaceTile).collect();
                // TODO: this may be redundant. Everything can be encompassed in
                // place tile
                events.push(Event::EndTurn);
                events
            }
            None => vec![Event::CantPlay],
        }
    }
}

static TILE_PATHS: [TilePath; 8] = [
    TilePath::Straight,
    TilePath::Diagonal,
    TilePath::Center90,
    TilePath::Corner90,
    TilePath::Left45,
    TilePath::Right45,
    TilePath::Left135,
    TilePath::Right135,
];
static ROTATIONS: [Rotation; 4] = [
    Rotation::None,
    Rotation::Clockwise90,
    Rotation::Clockwise180,
    Rotation::Clockwise270,
];

struct PotentialSetOfMoves {
    /// Save score for comparison against others
    score: TurnScore,
    /// Save moves for building final set of events
    placements: Vec<TilePlacementEvent>,
}

fn tile_paths_from_tile(t: Tile) -> Vec<TilePath> {
    match t {
        Tile::Straight => vec![TilePath::Straight],
        Tile::Diagonal => vec![TilePath::Diagonal],
        Tile::Center90 => vec![TilePath::Center90],
        Tile::Corner90 => vec![TilePath::Corner90],
        Tile::Left45 => vec![TilePath::Left45],
        Tile::Right45 => vec![TilePath::Right45],
        Tile::Left135 => vec![TilePath::Left135],
        Tile::Right135 => vec![TilePath::Right135],
        Tile::Universal => Vec::from(TILE_PATHS),
    }
}

impl Brute {
    fn best_moves(
        board: &Board,
        last_coordinates: Coordinates,
        last_offset: Offset,
        turn_score: TurnScore,
        tiles: &VecDeque<Tile>,
        placements: &Vec<TilePlacementEvent>,
    ) -> Option<PotentialSetOfMoves> {
        let next_coordinates = last_coordinates + last_offset;
        let mut potential_placements = Vec::new();
        for (idx, tile) in tiles.iter().enumerate() {
            for tile_path in tile_paths_from_tile(*tile) {
                let tile_path_type = if *tile == Tile::Universal {
                    TilePathType::Universal(tile_path)
                } else {
                    TilePathType::Normal(tile_path)
                };
                // TODO: some tiles are symmetrical and only have two effective rotations
                for rotation in ROTATIONS.iter() {
                    let placement = TilePlacementEvent {
                        coordinates: next_coordinates,
                        rotation: *rotation,
                        tile_path_type: tile_path_type.clone(),
                    };
                    if let Ok((next_coordinates, next_offset)) =
                        eval_placement((last_coordinates, last_offset), &placement)
                    {
                        if let Some(cell) = board.cell(next_coordinates) {
                            if !board.in_bounds(next_coordinates + next_offset) {
                                continue;
                            }
                            // Can't replay in same place
                            if placements
                                .iter()
                                .any(|placement| placement.coordinates == next_coordinates)
                                || !cell.is_empty()
                            {
                                continue;
                            }
                            let mut new_placements = placements.clone();
                            new_placements.push(placement);
                            let new_score = turn_score
                                + TurnScore::new(tile.score(), 0)
                                + cell.score()
                                + if tiles.len() == 1 {
                                    // Bonus for using all tiles
                                    TurnScore::new(20, 0)
                                } else {
                                    TurnScore::default()
                                };
                            let set_of_moves = PotentialSetOfMoves {
                                placements: new_placements.clone(),
                                score: new_score
                                    + Self::next_tile_adjustment(
                                        board,
                                        next_coordinates + next_offset,
                                    ),
                            };
                            potential_placements.push(set_of_moves);
                            if tiles.len() > 1 {
                                // Recurse
                                let mut rem_tiles = tiles.clone();
                                rem_tiles.remove(idx).unwrap();
                                Self::best_moves(
                                    board,
                                    next_coordinates,
                                    next_offset,
                                    new_score,
                                    &rem_tiles,
                                    &new_placements,
                                )
                                .map(|moves| potential_placements.push(moves));
                            }
                        }
                    }
                }
            }
        }
        potential_placements
            .into_iter()
            .max_by(|p1, p2| p1.score.cmp(&p2.score))
    }

    fn next_tile_adjustment(board: &Board, next_coordinates: Coordinates) -> TurnScore {
        // this should be a function of the number of players. In a two-player game, the
        // game is zero-sum
        const ADJUSTMENT: i16 = 2;
        match board.cell(next_coordinates) {
            Some(cell) if cell.bonus() > 0 => TurnScore::new(0, cell.bonus() / ADJUSTMENT),
            Some(cell) if cell.bonus() < 0 => TurnScore::new(-cell.bonus() / ADJUSTMENT, 0),
            _ => TurnScore::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::TilePlacement;

    #[test]
    fn maximizes_score() {
        let mut target = Brute::default();
        let board = Board::new();
        // Tiles to get to 60 bonus on first turn using all tiles
        let tiles = VecDeque::from(vec![
            Tile::Center90,
            Tile::Straight,
            Tile::Straight,
            Tile::Straight,
            Tile::Straight,
        ]);
        let moves = target.take_turn(&tiles, &board);
        assert_eq!(moves.len(), 6);
        matches!(&moves[0], Event::PlaceTile(tpe) if tpe.tile_path_type == TilePathType::Normal(TilePath::Straight));
        matches!(&moves[1], Event::PlaceTile(tpe) if tpe.tile_path_type == TilePathType::Normal(TilePath::Straight));
        matches!(&moves[2], Event::PlaceTile(tpe) if tpe.tile_path_type == TilePathType::Normal(TilePath::Center90));
        matches!(&moves[3], Event::PlaceTile(tpe) if tpe.tile_path_type == TilePathType::Normal(TilePath::Straight));
        matches!(&moves[4], Event::PlaceTile(tpe) if tpe.tile_path_type == TilePathType::Normal(TilePath::Straight));
        matches!(&moves[5], Event::EndTurn);
    }

    #[test]
    fn second_turn() {
        let mut target = Brute::default();
        let mut board = Board::with_last_placement(Coordinates(10, 0), Offset(0, 1));
        board
            .place_tile(
                Coordinates(10, 0),
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        // Tiles to get to 60 bonus on first turn using all tiles
        let tiles = VecDeque::from(vec![
            Tile::Center90,
            Tile::Center90,
            // Can't use these
            Tile::Diagonal,
            Tile::Diagonal,
            Tile::Diagonal,
        ]);

        let moves = target.take_turn(&tiles, &board);
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn ignore_out_of_bounds_paths() {}
}
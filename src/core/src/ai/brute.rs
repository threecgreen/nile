use super::CPUPlayer;
use crate::board::Board;
use crate::log::TilePlacementEvent;
use crate::path::{eval_placement, Offset, TilePath, TilePathType, TILE_PATHS};
use crate::player::TileArray;
use crate::score::TurnScore;
use crate::tile::{Coordinates, Tile, ROTATIONS};

#[derive(Debug)]
pub struct Brute {
    player_count: usize,
}

impl Brute {
    pub fn new(player_count: usize) -> Self {
        Self { player_count }
    }
}

impl CPUPlayer for Brute {
    fn take_turn(
        &mut self,
        tiles: &TileArray,
        board: &Board,
        score: i16,
        other_scores: Vec<i16>,
    ) -> Vec<Vec<TilePlacementEvent>> {
        let last_placement = board.last_placement();
        let mut all_moves = self.ranked_moves(
            board,
            score,
            &other_scores,
            last_placement.0,
            last_placement.1,
            TurnScore::default(),
            tiles,
            &Vec::new(),
        );
        // Compare `y` against `x` for descending order
        all_moves.sort_by(|x, y| y.score.cmp(&x.score));
        all_moves
            .into_iter()
            .map(|potential_moves| potential_moves.placements)
            .collect()
    }
}

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
    #[allow(clippy::too_many_arguments)]
    fn ranked_moves(
        &self,
        board: &Board,
        score: i16,
        other_scores: &[i16],
        last_coordinates: Coordinates,
        last_offset: Offset,
        turn_score: TurnScore,
        tiles: &TileArray,
        placements: &[TilePlacementEvent],
    ) -> Vec<PotentialSetOfMoves> {
        let next_coordinates = last_coordinates + last_offset;
        let mut potential_placements = Vec::new();
        for (idx, tile) in tiles.iter().enumerate() {
            for tile_path in tile_paths_from_tile(*tile) {
                let tile_path_type = if *tile == Tile::Universal {
                    TilePathType::Universal(tile_path)
                } else {
                    TilePathType::Normal(tile_path)
                };
                // Straight and diagonal only have two effective rotations because
                // they're symettrical
                let rotations =
                    if tile_path == TilePath::Straight || tile_path == TilePath::Diagonal {
                        &ROTATIONS[..2]
                    } else {
                        &ROTATIONS[..]
                    };
                for rotation in rotations.iter() {
                    let placement = TilePlacementEvent {
                        coordinates: next_coordinates,
                        rotation: *rotation,
                        tile_path_type,
                    };
                    if let Ok((next_coordinates, next_offset)) =
                        eval_placement((last_coordinates, last_offset), &placement)
                    {
                        if let Some(cell) = board.cell(next_coordinates) {
                            if !board.in_bounds(next_coordinates + next_offset)
                                // If this is an end game cell, the next coordinates don't need
                                // to be in bounds
                                && !board.is_end_game_cell(next_coordinates)
                            {
                                continue;
                            }
                            // Can't replay in same place
                            let has_replayed_in_cell = placements.iter().any(|placement| {
                                placement.coordinates == next_coordinates
                                    || placement.coordinates == next_coordinates + next_offset
                            }) || !cell.is_empty()
                                || board.has_tile(next_coordinates + next_offset);
                            if has_replayed_in_cell {
                                continue;
                            }
                            if board.no_crossover(next_coordinates, next_offset).is_err() {
                                continue;
                            }
                            // clone
                            let mut new_placements = placements.to_owned();
                            new_placements.push(placement);
                            let new_score = turn_score
                                + TurnScore::from(tile.score())
                                + cell.score()
                                + if tiles.len() == 1 {
                                    // TODO: fix when fewer than 5 tiles in rack
                                    // Bonus for using all tiles
                                    TurnScore::from(20)
                                } else {
                                    TurnScore::default()
                                };
                            let end_game_adj = match Brute::end_game_adjustment(
                                score,
                                other_scores,
                                board,
                                new_score,
                                &new_placements,
                                (next_coordinates, next_offset),
                            ) {
                                Ok(adj) => adj,
                                Err(()) => {
                                    continue;
                                }
                            };
                            let set_of_moves = PotentialSetOfMoves {
                                placements: new_placements.clone(),
                                score: new_score
                                    + self.next_tile_adjustment(
                                        board,
                                        next_coordinates + next_offset,
                                    )
                                    + end_game_adj,
                            };
                            potential_placements.push(set_of_moves);
                            if tiles.len() > 1
                                // Don't try to play more tiles if this set of moves
                                // will already end the game
                                && !board.is_end_game_cell(next_coordinates)
                            {
                                // Recurse
                                let mut rem_tiles = tiles.clone();
                                rem_tiles.remove(idx);
                                potential_placements.extend(self.ranked_moves(
                                    board,
                                    score,
                                    other_scores,
                                    next_coordinates,
                                    next_offset,
                                    new_score,
                                    &rem_tiles,
                                    &new_placements,
                                ));
                            }
                        }
                    }
                }
            }
        }
        potential_placements
    }

    fn next_tile_adjustment(&self, board: &Board, next_coordinates: Coordinates) -> TurnScore {
        // this should be a function of the number of players. In a two-player game, the
        // game is zero-sum
        match board.cell(next_coordinates) {
            // when player count is 2, it's a zero-sum game, so forced
            // are as valuable as bonuses for the player
            Some(cell) => -cell.score() * 2 / self.player_count as i16,
            _ => TurnScore::default(),
        }
    }

    fn end_game_adjustment(
        score: i16,
        other_player_scores: &[i16],
        board: &Board,
        turn_score: TurnScore,
        tile_placements: &[TilePlacementEvent],
        last_placement: (Coordinates, Offset),
    ) -> Result<TurnScore, ()> {
        let end_of_game_cell_count = tile_placements
            .iter()
            .filter(|p| board.is_end_game_cell(p.coordinates))
            .count();
        let ends_game = Board::validate_end_of_game_cells(end_of_game_cell_count, last_placement)
            .map_err(|_| ())?;
        let total_score = score + turn_score.score();
        let rank = other_player_scores
            .iter()
            .filter(|score| **score >= total_score)
            .count()
            + 1;
        match (ends_game, rank) {
            // Highly incentivize ending the game when winning
            (true, 1) => Ok(TurnScore::from(1000)),
            // Want to penalize ending the game without winning
            (true, _) => Ok(TurnScore::from(-100)),
            (false, _) => Ok(TurnScore::default()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::TilePlacement;
    use crate::tile::Rotation;

    use smallvec::smallvec;

    #[test]
    fn maximizes_score() {
        let mut target = Brute::new(2);
        let board = Board::new();
        // Tiles to get to 60 bonus on first turn using all tiles
        let tiles = smallvec![
            Tile::Right45,
            Tile::Left45,
            Tile::Straight,
            Tile::Straight,
            Tile::Straight,
        ];
        let all_moves = target.take_turn(&tiles, &board, 0, vec![0]);
        let moves = all_moves.first().unwrap();
        assert_eq!(moves.len(), 5);
        assert!(matches!(
            &moves[0],
            TilePlacementEvent {
                coordinates: Coordinates(10, 0),
                rotation: Rotation::None,
                tile_path_type: TilePathType::Normal(TilePath::Straight)
            }
        ));
        assert!(matches!(
            &moves[1],
            TilePlacementEvent {
                coordinates: Coordinates(10, 1),
                rotation: Rotation::Clockwise90,
                tile_path_type: TilePathType::Normal(TilePath::Right45)
            }
        ));
        assert!(matches!(
            &moves[2],
            TilePlacementEvent {
                coordinates: Coordinates(11, 2),
                rotation: Rotation::None,
                tile_path_type: TilePathType::Normal(TilePath::Left45)
            }
        ));
        assert!(matches!(
            &moves[3],
            TilePlacementEvent {
                coordinates: Coordinates(12, 2),
                rotation: Rotation::Clockwise90,
                tile_path_type: TilePathType::Normal(TilePath::Straight)
            }
        ));
        assert!(matches!(
            &moves[4],
            TilePlacementEvent {
                coordinates: Coordinates(13, 2),
                rotation: Rotation::Clockwise90,
                tile_path_type: TilePathType::Normal(TilePath::Straight)
            }
        ));
    }

    #[test]
    fn second_turn() {
        let mut target = Brute::new(2);
        let mut board = Board::with_last_placement(Coordinates(10, 0), Offset(0, 1));
        board
            .place_tile(
                Coordinates(10, 0),
                TilePlacement::new(TilePathType::Normal(TilePath::Straight), Rotation::None),
            )
            .unwrap();
        // Tiles to get to 60 bonus on first turn using all tiles
        let tiles = smallvec![
            Tile::Center90,
            Tile::Center90,
            // Can't use these
            Tile::Diagonal,
            Tile::Diagonal,
            Tile::Diagonal,
        ];

        let all_moves = target.take_turn(&tiles, &board, 30, vec![50]);
        let moves = all_moves.first().unwrap();
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn ignore_out_of_bounds_paths() {
        let mut target = Brute::new(2);
        let board = Board::with_last_placement(Coordinates(19, 0), Offset(1, 0));
        let tiles = smallvec![
            Tile::Straight,
            Tile::Straight,
            Tile::Diagonal,
            Tile::Diagonal,
            Tile::Straight,
        ];

        let moves = target.take_turn(&tiles, &board, 0, vec![146]);
        assert!(moves.is_empty());
    }

    #[test]
    fn cpu_should_end_game_when_winning() {
        let mut target = Brute::new(3);
        let board = Board::with_last_placement(Coordinates(10, 19), Offset(0, 1));
        let tiles = smallvec![
            Tile::Straight,
            Tile::Right135,
            Tile::Right45,
            Tile::Right45,
            Tile::Straight,
        ];

        // Optimal moves should place player in lead
        let all_moves = target.take_turn(&tiles, &board, 400, vec![900, 885]);
        let moves = all_moves.first().unwrap();
        assert_eq!(moves[0].coordinates, Coordinates(10, 20));
        assert_eq!(
            moves[0].tile_path_type,
            TilePathType::Normal(TilePath::Straight)
        );
        assert_eq!(moves[1].coordinates, Coordinates(10, 21));
        assert_eq!(
            moves[1].tile_path_type,
            TilePathType::Normal(TilePath::Straight)
        );
    }
}

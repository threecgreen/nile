mod brute;

use crate::board::Board;
use crate::log::TilePlacementEvent;
use crate::tile::Tile;

use std::collections::VecDeque;

/// An automated player to compete with human players or other `CPUPlayer`s
pub trait CPUPlayer: std::fmt::Debug {
    fn take_turn(
        &mut self,
        tiles: &VecDeque<Tile>,
        board: &Board,
        score: i16,
        other_scores: Vec<i16>,
    ) -> Option<Vec<TilePlacementEvent>>;
}

pub use brute::Brute;

use crate::tile::{Tile, TileBox};
use crate::score::TurnScore;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Player {
    name: String,
    tile_rack: VecDeque<Tile>,
    /// Scores of completed turns
    scores: Vec<TurnScore>,
    /// Scores of current turn
    current_turn_score: TurnScore,
}

static MAX_TILES: usize = 5;

impl Player {
    pub fn new(name: String, tile_box: &mut TileBox) -> Self {
        // TODO: handle case where box is empty
        let mut tile_rack = VecDeque::with_capacity(MAX_TILES);
        Self::fill_rack(&mut tile_rack, tile_box);
        Self {
            name,
            tile_rack,
            scores: Vec::new(),
            current_turn_score: TurnScore::default(),
        }
    }

    pub fn end_turn(&mut self, tile_box: &mut TileBox) {
        Self::fill_rack(&mut self.tile_rack, tile_box);
        self.scores.push(self.current_turn_score);
        self.current_turn_score = TurnScore::default();
    }

    pub fn rack_is_empty(&self) -> bool {
        self.tile_rack.is_empty()
    }

    fn fill_rack(tile_rack: &mut VecDeque<Tile>, tile_box: &mut TileBox) {
        while tile_rack.len() < MAX_TILES {
            if let Some(tile) = tile_box.draw() {
                tile_rack.push_back(tile);
            } else {
                break;
            }
        }
    }

    pub fn tiles(&self) -> &VecDeque<Tile> {
        &self.tile_rack
    }

    /// The player is placing a tile of variant `tile`. Validate the player has at least one of
    /// these tiles and remove it from their rack.
    pub fn place_tile(&mut self, tile: Tile) -> Option<Tile> {
        self.tile_rack.iter().position(|t| t == t)
            .and_then(|idx| self.tile_rack.remove(idx))
    }

    /// The player removed a tile from the board is returning it to their rack
    pub fn return_tile(&mut self, tile: Tile) {
        self.tile_rack.push_back(tile);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modify the current turn score and return the updated turn score
    pub fn add_score(&mut self, score: TurnScore) -> TurnScore {
        self.current_turn_score += score;
        self.current_turn_score
    }

    pub fn total_score(&self) -> i16 {
        self.scores.iter().fold(0, |total, score| total + score.score())
    }

    pub fn discard_tiles(&mut self) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(self.tile_rack.len());
        while let Some(tile) = self.tile_rack.remove(0) {
            tiles.push(tile);
        }
        tiles
    }
}

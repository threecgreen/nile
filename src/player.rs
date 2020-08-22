use crate::score::TurnScore;
use crate::tile::{Tile, TileBox};

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
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

    pub fn end_turn(&mut self, tile_box: &mut TileBox) -> TurnScore {
        if self.tile_rack.is_empty() {
            // Bonus for using all tiles
            self.add_score(TurnScore::new(20, 0));
        }
        let final_turn_score = self.current_turn_score;
        Self::fill_rack(&mut self.tile_rack, tile_box);
        self.scores.push(final_turn_score);
        self.current_turn_score = TurnScore::default();
        final_turn_score
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
        self.tile_rack
            .iter()
            .position(|t| *t == tile)
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
        self.scores
            .iter()
            .fold(0, |total, score| total + score.score())
    }

    pub fn discard_tiles(&mut self) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(self.tile_rack.len());
        while let Some(tile) = self.tile_rack.remove(0) {
            tiles.push(tile);
        }
        tiles
    }
}

pub mod wasm {
    use super::Player;

    use js_sys::Array;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Player {
        pub fn get_name(&self) -> JsValue {
            JsValue::from(self.name())
        }

        pub fn get_tiles(&self) -> Array {
            self.tiles()
                .into_iter()
                .map(|t| JsValue::from_serde(t).unwrap())
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (TileBox, Player) {
        let mut tile_box = TileBox::new();
        let target = Player::new("Test".to_owned(), &mut tile_box);
        (tile_box, target)
    }

    #[test]
    fn adding_score_changes_score() {
        let (_, mut target) = setup();
        let turn_score = TurnScore::new(7, 7);
        let updated_score = target.add_score(turn_score);
        assert_eq!(updated_score, turn_score);
    }

    /// Adding opposite scores returns the original score
    #[test]
    fn opposite_returns_original_score() {
        let (_, mut target) = setup();
        let score = TurnScore::new(30, 10);
        let mut current_score = target.add_score(score);
        assert_eq!(current_score, score);
        current_score = target.add_score(-score);
        assert_eq!(current_score, TurnScore::new(0, 0));
    }

    #[test]
    fn end_turn_updates_scores() {
        let (mut tile_box, mut target) = setup();
        assert_eq!(target.scores, []);
        target.add_score(TurnScore::new(10, 10));
        let current_score = target.add_score(TurnScore::new(25, 60));
        assert_eq!(current_score, TurnScore::new(35, 70));
        assert_eq!(current_score, target.end_turn(&mut tile_box));
        assert_eq!(target.scores, vec![TurnScore::new(35, 70)]);
    }

    #[test]
    fn end_turn_adds_used_all_tiles_bonus() {
        let (mut tile_box, mut target) = setup();
        for tile in target.tiles().clone() {
            target.place_tile(tile);
        }
        assert_eq!(TurnScore::new(20, 0), target.end_turn(&mut tile_box));
    }
}

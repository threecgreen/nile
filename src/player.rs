use crate::score::TurnScore;
use crate::tile::{Tile, TileBox};

use smallvec::SmallVec;
use wasm_bindgen::prelude::*;

const MAX_TILES: usize = 5;

pub type TileArray = SmallVec<[Tile; MAX_TILES]>;

/// Holds all data related to a single player
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    tile_rack: TileArray,
    /// Scores of completed turns
    scores: Vec<TurnScore>,
    /// Scores of current turn
    current_turn_score: TurnScore,
    /// Computer-controlled player
    is_cpu: bool,
}

impl Player {
    pub fn new(name: String, tile_box: &mut TileBox, is_cpu: bool) -> Self {
        let mut tile_rack = TileArray::new();
        Self::fill_rack(&mut tile_rack, tile_box);
        Self {
            name,
            tile_rack,
            scores: Vec::new(),
            current_turn_score: TurnScore::default(),
            is_cpu,
        }
    }

    /// Refill the player's `tile_rack` and return the total score of the tiles they played in
    /// the current turn.
    pub fn end_turn(&mut self, tile_box: &mut TileBox) -> TurnScore {
        if self.tile_rack.is_empty() {
            // TODO: this should possibly only apply if the player began
            // their turn with 5 tiles
            // Bonus for using all tiles
            self.add_score(TurnScore::from(20));
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

    fn fill_rack(tile_rack: &mut TileArray, tile_box: &mut TileBox) {
        while tile_rack.len() < MAX_TILES {
            if let Some(tile) = tile_box.draw() {
                tile_rack.push(tile);
            } else {
                break;
            }
        }
    }

    pub fn tiles(&self) -> &TileArray {
        &self.tile_rack
    }

    /// The player is placing a tile of variant `tile`. Validate the player has at least one of
    /// these tiles and remove it from their rack.
    pub fn place_tile(&mut self, tile: Tile) -> Option<Tile> {
        self.tile_rack
            .iter()
            .position(|t| *t == tile)
            .and_then(|idx| {
                if idx < self.tile_rack.len() {
                    Some(self.tile_rack.remove(idx))
                } else {
                    None
                }
            })
    }

    /// The player can't play any tiles and is ending their turn. Discards all their current tiles
    /// and refills their `tile_rack` from `tile_box`.
    pub fn cant_play(&mut self, tile_box: &mut TileBox) -> TurnScore {
        let tiles = self.discard_tiles();
        let tile_score = tiles.iter().fold(0, |acc, t| acc + t.score());
        let turn_score = TurnScore {
            add: 0,
            sub: tile_score,
        };
        self.add_score(turn_score);
        tile_box.discard(tiles);
        Self::fill_rack(&mut self.tile_rack, tile_box);
        self.scores.push(turn_score);
        self.current_turn_score = TurnScore::default();
        turn_score
    }

    /// The player removed a tile from the board is returning it to their rack
    pub fn return_tile(&mut self, tile: Tile) {
        self.tile_rack.push(tile);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modify the current turn score and return the updated turn score
    pub fn add_score(&mut self, score: TurnScore) -> TurnScore {
        self.current_turn_score += score;
        self.current_turn_score
    }

    fn discard_tiles(&mut self) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(self.tile_rack.len());
        while let Some(tile) = self.tile_rack.pop() {
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
                .iter()
                .map(|t| JsValue::from_f64(*t as i32 as f64))
                .collect()
        }

        pub fn total_score(&self) -> i16 {
            self.scores
                .iter()
                .fold(0, |total, score| total + score.score())
        }

        pub fn is_cpu(&self) -> bool {
            self.is_cpu
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (TileBox, Player) {
        let mut tile_box = TileBox::default();
        let target = Player::new("Test".to_owned(), &mut tile_box, false);
        (tile_box, target)
    }

    #[test]
    fn adding_score_changes_score() {
        let (_, mut target) = setup();
        let turn_score = TurnScore { add: 7, sub: 7 };
        let updated_score = target.add_score(turn_score);
        assert_eq!(updated_score, turn_score);
    }

    /// Adding opposite scores returns the original score
    #[test]
    fn opposite_returns_original_score() {
        let (_, mut target) = setup();
        let score = TurnScore { add: 30, sub: 10 };
        let mut current_score = target.add_score(score);
        assert_eq!(current_score, score);
        current_score = target.add_score(-score);
        assert_eq!(current_score, TurnScore { add: 0, sub: 0 });
    }

    #[test]
    fn end_turn_updates_scores() {
        let (mut tile_box, mut target) = setup();
        assert_eq!(target.scores, []);
        target.add_score(TurnScore { add: 10, sub: 10 });
        let current_score = target.add_score(TurnScore { add: 25, sub: 60 });
        assert_eq!(current_score, TurnScore { add: 35, sub: 70 });
        assert_eq!(current_score, target.end_turn(&mut tile_box));
        assert_eq!(target.scores, vec![TurnScore { add: 35, sub: 70 }]);
    }

    #[test]
    fn end_turn_adds_used_all_tiles_bonus() {
        let (mut tile_box, mut target) = setup();
        for tile in target.tiles().clone() {
            target.place_tile(tile);
        }
        assert_eq!(
            TurnScore { add: 20, sub: 0 },
            target.end_turn(&mut tile_box)
        );
    }

    #[test]
    fn cant_play_score() {
        let (mut tile_box, mut target) = setup();
        assert_eq!(target.scores, []);
        let expected_score = target.tiles().iter().fold(0i16, |acc, t| acc - t.score());
        let res = target.cant_play(&mut tile_box);
        assert_eq!(res.score(), expected_score);
        // Previously can't play would return the correct score but store a score with an added 20
        // points for using all tiles.
        assert_eq!(
            target.scores,
            vec![TurnScore {
                add: 0,
                sub: -expected_score
            }]
        );
    }
}

use crate::tile::{Tile, TileBox};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Player {
    name: String,
    tile_rack: VecDeque<Tile>,
    scores: Vec<i16>,
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
        }
    }

    pub fn end_turn(&mut self, tile_box: &mut TileBox) {
        Self::fill_rack(&mut self.tile_rack, tile_box);
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

    pub fn get_tile(&self, index: usize) -> Option<&Tile> {
        self.tile_rack.get(index)
    }

    pub fn remove_tile(&mut self, index: usize) -> Option<Tile> {
        self.tile_rack.remove(index)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_score(&mut self, score: i16) {
        self.scores.push(score);
    }

    pub fn total_score(&self) -> i16 {
        self.scores.iter().fold(0, |total, score| total + score)
    }

    pub fn discard_tiles(&mut self) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(self.tile_rack.len());
        while let Some(tile) = self.tile_rack.remove(0) {
            tiles.push(tile);
        }
        tiles
    }
}

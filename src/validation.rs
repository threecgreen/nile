use crate::board::Board;
use crate::nile::{Move, TilePlacement};

pub fn is_valid(board: &Board, tile_placement: &TilePlacement) -> Result<(), String> {
    if !board.get_cell(tile_placement.row, tile_placement.column).is_empty() {
        return Err("Cell already contains a tile".to_owned())
    }

    Ok(())
}

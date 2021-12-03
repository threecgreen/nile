use std::collections::HashSet;

use crate::Coordinates;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Msg(String),
    Cell(CellError),
}

impl Error {
    pub fn cell(coordinates: Coordinates, msg: String) -> Self {
        Self::Cell(CellError::new(coordinates, msg))
    }

    pub fn cells(coordinates: HashSet<Coordinates>, msg: String) -> Self {
        Self::Cell(CellError::new_many(coordinates, msg))
    }
}

/// Error specific to a cell on the board
#[derive(Debug)]
pub struct CellError {
    pub coordinates: HashSet<Coordinates>,
    pub msg: String,
}

impl CellError {
    pub fn new(coordinates: Coordinates, msg: String) -> Self {
        let mut set = HashSet::new();
        set.insert(coordinates);
        Self {
            coordinates: set,
            msg,
        }
    }

    pub fn new_many(coordinates: HashSet<Coordinates>, msg: String) -> Self {
        Self { coordinates, msg }
    }
}

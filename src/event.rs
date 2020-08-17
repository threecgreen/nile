use crate::tile::{self, Coordinates, Tile};

use serde::Deserialize;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct TilePlacement {
    pub tile: Tile,
    pub coordinates: Coordinates,
    pub rotation: tile::Rotation,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rotation {
    pub coordinates: Coordinates,
    pub rotation: tile::Rotation,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Event {
    PlaceTile(TilePlacement),
    RotateTile(Rotation),
    RemoveTile(Coordinates),
    // MoveTile({old: Coordinates, new: Coordinates})
    Undo,
    Redo,
    CantPlay,
    EndTurn,
}

/// Event log
#[derive(Debug)]
pub struct Log {
    events: Vec<Event>,
    redo_events: Vec<Event>,
    undo_events: Vec<Event>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            redo_events: Vec::new(),
            undo_events: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::RotateTile(_) | Event::PlaceTile(_) | Event::RemoveTile(_) => {
                self.undo_events.push(event);
                None
            }
            Event::Undo => {
                let event = self.undo_events.pop();
                match event {
                    Some(event) => {
                        self.redo_events.push(event.clone());
                        Some(event)
                    }
                    None => None,
                }
            }
            Event::Redo => {
                let event = self.redo_events.pop();
                match event {
                    Some(event) => {
                        self.undo_events.push(event.clone());
                        Some(event)
                    }
                    None => None,
                }
            }
            Event::CantPlay | Event::EndTurn => {
                self.redo_events.clear();
                self.events.append(&mut self.undo_events);
                None
            }
        }
    }

    /// Coordinates of all cells that have been modified this turn
    pub fn current_turn_coordinates(&self) -> HashSet<Coordinates> {
        self.undo_events
            .iter()
            .filter_map(|e| {
                // TODO: This could be optimized to determine which coordinates still have tiles
                match e {
                    Event::RotateTile(r) => Some(r.coordinates),
                    Event::PlaceTile(tp) => Some(tp.coordinates),
                    Event::RemoveTile(c) => Some(*c),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn can_undo(&self) -> bool {
        self.undo_events.len() > 0
    }

    pub fn can_redo(&self) -> bool {
        self.redo_events.len() > 0
    }
}

use crate::tile::{self, Coordinates, Tile};

#[derive(Copy, Clone, Debug)]
pub struct TilePlacementEvent {
    pub tile: Tile,
    pub coordinates: Coordinates,
    pub rotation: tile::Rotation,
}

#[derive(Clone, Debug)]
pub struct Rotation {
    pub coordinates: Coordinates,
    pub rotation: tile::Rotation,
}

#[derive(Clone, Debug)]
pub struct RevertableEvent<T: Clone> {
    pub old: T,
    pub new: T,
}

type RotationEvent = RevertableEvent<Rotation>;
type MoveTileEvent = RevertableEvent<Coordinates>;

/// Internal representation of a user event. All information necessary for
/// undoing a `Event` is self-contained.
#[derive(Clone, Debug)]
pub enum Event {
    PlaceTile(TilePlacementEvent),
    RotateTile(RotationEvent),
    RemoveTile(TilePlacementEvent),
    MoveTile(MoveTileEvent),
    CantPlay,
    EndTurn,
}

impl Event {
    fn revert(&self) -> Option<Event> {
        match self {
            Event::PlaceTile(tile_placement) => Some(Event::RemoveTile(tile_placement.clone())),
            Event::RotateTile(rotation) => Some(Event::RotateTile(RotationEvent {
                old: rotation.new.clone(),
                new: rotation.old.clone(),
            })),
            Event::RemoveTile(tile_placement) => Some(Event::PlaceTile(tile_placement.clone())),
            Event::MoveTile(move_tile) => Some(Event::MoveTile(MoveTileEvent {
                new: move_tile.old,
                old: move_tile.new,
            })),
            // Can't undo end of turn
            Event::CantPlay | Event::EndTurn => None,
        }
    }
}

/// Game event log
#[derive(Debug)]
pub struct Log {
    /// Immutable events of past turns
    events: Vec<Event>,
    /// Events from this turn that can be redone
    redo_events: Vec<Event>,
    /// Events from this turn that can be undone
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

    pub fn undo(&mut self) -> Option<Event> {
        self.undo_events.pop().and_then(|e| e.revert()).map(|e| {
            self.redo_events.push(e.clone());
            e
        })
    }

    pub fn redo(&mut self) -> Option<Event> {
        self.redo_events.pop()
    }

    pub fn place_tile(&mut self, tile: Tile, coordinates: Coordinates, rotation: tile::Rotation) {
        self.undo_events.push(Event::PlaceTile(TilePlacementEvent {
            tile,
            coordinates,
            rotation,
        }));
    }

    pub fn rotate_tile(
        &mut self,
        coordinates: Coordinates,
        old_rotation: tile::Rotation,
        new_rotation: tile::Rotation,
    ) {
        self.undo_events.push(Event::RotateTile(RotationEvent {
            old: Rotation {
                coordinates,
                rotation: old_rotation,
            },
            new: Rotation {
                coordinates,
                rotation: new_rotation,
            },
        }));
    }

    pub fn remove_tile(&mut self, tile: Tile, coordinates: Coordinates, rotation: tile::Rotation) {
        self.undo_events.push(Event::RemoveTile(TilePlacementEvent {
            tile,
            coordinates,
            rotation,
        }));
    }

    pub fn move_tile(&mut self, old_coordinates: Coordinates, new_coordinates: Coordinates) {
        self.undo_events.push(Event::MoveTile(MoveTileEvent {
            old: old_coordinates,
            new: new_coordinates,
        }));
    }

    pub fn cant_play(&mut self) {
        self.undo_events.push(Event::CantPlay);
        self.events.append(&mut self.undo_events);
    }

    pub fn end_turn(&mut self) {
        self.undo_events.push(Event::EndTurn);
        self.events.append(&mut self.undo_events);
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_events.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_events.is_empty()
    }
}

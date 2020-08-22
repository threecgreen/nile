use crate::path::{TilePath, TilePathType};
use crate::tile::{self, Coordinates};

#[derive(Clone, Debug)]
pub struct TilePlacementEvent {
    pub tile_path_type: TilePathType,
    pub coordinates: Coordinates,
    pub rotation: tile::Rotation,
}

#[derive(Clone, Debug)]
pub struct Rotation {
    pub coordinates: Coordinates,
    pub rotation: tile::Rotation,
}

#[derive(Clone, Debug)]
pub struct UpdateUniversalPathEvent {
    pub coordinates: Coordinates,
    pub old_tile_path: TilePath,
    pub new_tile_path: TilePath,
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
    UpdateUniversalPath(UpdateUniversalPathEvent),
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
            Event::UpdateUniversalPath(update) => {
                Some(Event::UpdateUniversalPath(UpdateUniversalPathEvent {
                    old_tile_path: update.new_tile_path,
                    new_tile_path: update.old_tile_path,
                    coordinates: update.coordinates,
                }))
            }
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

    pub fn begin_undo(&mut self) -> Option<Event> {
        self.undo_events.pop().and_then(|e| {
            self.redo_events.push(e.clone());
            e.revert()
        })
    }

    pub fn end_undo(&mut self) {
        self.undo_events.pop();
    }

    pub fn redo(&mut self) -> Option<Event> {
        self.redo_events.pop()
    }

    pub fn place_tile(
        &mut self,
        tile_path_type: TilePathType,
        coordinates: Coordinates,
        rotation: tile::Rotation,
    ) {
        self.undo_events.push(Event::PlaceTile(TilePlacementEvent {
            tile_path_type,
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

    pub fn remove_tile(
        &mut self,
        tile_path_type: TilePathType,
        coordinates: Coordinates,
        rotation: tile::Rotation,
    ) {
        self.undo_events.push(Event::RemoveTile(TilePlacementEvent {
            tile_path_type,
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

    pub fn update_universal_path(
        &mut self,
        coordinates: Coordinates,
        old_tile_path: TilePath,
        new_tile_path: TilePath,
    ) {
        self.undo_events
            .push(Event::UpdateUniversalPath(UpdateUniversalPathEvent {
                coordinates,
                new_tile_path,
                old_tile_path,
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

    /// Whether there are events that can be undone
    pub fn can_undo(&self) -> bool {
        !self.undo_events.is_empty()
    }

    /// Whether there are events that can be redone
    pub fn can_redo(&self) -> bool {
        !self.redo_events.is_empty()
    }

    /// Check if a cell (specified by a set of coordinates) was changed during
    /// the current turn.
    pub fn cell_changed_in_turn(&self, coordinates: Coordinates) -> bool {
        // Don't need to validate if there's still a tile there because that will be handled by
        // `crate::board::Board`
        self.undo_events.iter().rev().any(|e| match e {
            Event::PlaceTile(tpe) | Event::RemoveTile(tpe) if tpe.coordinates == coordinates => {
                true
            }
            Event::MoveTile(mte) if mte.new == coordinates => true,
            Event::RotateTile(rte) if rte.new.coordinates == coordinates => true,
            _ => false,
        }) || self
            .redo_events
            .iter()
            .any(|e| Self::event_matches_coordinates(e, coordinates))
    }

    fn event_matches_coordinates(event: &Event, coordinates: Coordinates) -> bool {
        match event {
            Event::PlaceTile(tpe) | Event::RemoveTile(tpe) if tpe.coordinates == coordinates => {
                true
            }
            Event::MoveTile(mte) if mte.new == coordinates => true,
            Event::RotateTile(rte) if rte.new.coordinates == coordinates => true,
            _ => false,
        }
    }
}

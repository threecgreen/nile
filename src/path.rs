use crate::log::TilePlacementEvent;
use crate::tile::{Coordinates, Offset, Rotation, Tile};

pub fn new_offset(
    prev: (Coordinates, Offset),
    placement: &TilePlacementEvent,
) -> Result<Offset, String> {
    // Handle board boundaries elsewhere
    if prev.0 + prev.1 != placement.coordinates {
        return Err("Coordinates dont't align with the rest of the river".to_owned());
    }
    let offsets: Vec<Offset> = placement
        .tile
        .directions()
        .iter()
        .map(|d| d.into_offset().rotate(placement.rotation))
        .collect();
    let rev_offset = offsets
        .iter()
        .find(|o| placement.coordinates + **o == prev.0)
        .ok_or_else(|| "Tile and rotation don't align with the rest of the river".to_owned())?;
    // TODO: fix for universal
    offsets
        .iter()
        .find(|o| *o != rev_offset)
        .ok_or_else(|| "No valid output offset".to_owned())
        .map(|o| *o)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrong_coordinates() {
        let res = new_offset(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile: Tile::Diagonal,
                rotation: Rotation::Clockwise90,
                coordinates: Coordinates(0, 1),
            },
        );
        matches!(res, Err(msg) if msg.starts_with("Coordinates"));
    }

    #[test]
    fn wrong_tile() {
        let res = new_offset(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile: Tile::Straight,
                rotation: Rotation::None,
                coordinates: Coordinates(1, 1),
            },
        );
        matches!(res, Err(msg) if msg.starts_with("Tile and rotation"));
    }

    #[test]
    fn wrong_rotation() {
        let res = new_offset(
            (Coordinates(0, 0), Offset(1, 1)),
            &TilePlacementEvent {
                tile: Tile::Diagonal,
                rotation: Rotation::Clockwise90,
                coordinates: Coordinates(1, 1),
            },
        );
        matches!(res, Err(msg) if msg.starts_with("Tile and rotation"));
    }

    #[test]
    fn ok_new_offset() {}
}

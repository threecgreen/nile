import { Tile, Coordinates, Rotation } from "nile";

export class PlaceTileEvent {
    public PlaceTile: {
        tile: string;
        coordinates: [number, number];
        rotation: string;
    }

    public constructor(tile: Tile, row: number, column: number, rotation: Rotation) {
        this.PlaceTile = {
            tile: Tile[tile],
            coordinates: [row, column],
            rotation: Rotation[rotation],
        };
    }
}

export class RotateTileEvent {
    public RotateTile: {
        coordinates: [number, number];
        rotation: string;
    };

    public constructor(row: number, column: number, rotation: Rotation) {
        this.RotateTile = {
            coordinates: [row, column],
            rotation: Rotation[rotation],
        };
    }
}

export class RemoveTileEvent {
    public RemoveTile: {
        coordinates: [number, number];
    };

    public constructor(row: number, column: number) {
        this.RemoveTile = {
            coordinates: [row, column],
        };
    }
}

export const UndoEvent = "Undo";
export const RedoEvent = "Redo";
export const CantPlayEvent = "CantPlay";
export const EndTurnEvent = "EndTurn";

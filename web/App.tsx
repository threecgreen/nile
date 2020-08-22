import { Board } from "components/Board";
import { Button } from "components/Button";
import { useUndoReducer } from "lib/hooks";
import { initState, reducer } from "lib/state";
import { Coordinates, Rotation, TilePathType, Tile, TilePath } from "nile";
import React from "react";
import { mod } from "lib/utils";
import { Player } from "components/Player";

export const App: React.FC = () => {
    // State
    const players = ["player1", "player2"];
    const [state, dispatch] = useUndoReducer(reducer, () => initState(players))

    // Event handlers
    const onDropFromRack = (row: number, column: number) => {
        if (state.draggedTile !== null) {
            // Move this to another file
            try {
                const rotation = Rotation.None;
                const tilePathType = state.draggedTile.isUniversal
                    ? TilePathType.universal(state.draggedTile.tilePath)
                    : TilePathType.normal(state.draggedTile.tilePath);
                // TODO: support
                const score = state.nile.place_tile(tilePathType, new Coordinates(row, column), rotation);
                dispatch({
                    type: "placeTile",
                    draggedTile: state.draggedTile, coordinates: [row, column],
                    rotation, score,
                });
            } catch (e) {
                console.error(e);
            }
        }
    }
    const onRotate = (isClockwise: boolean) => {
        if(state.selectedTile) {
            const [row, column] = state.selectedTile;
            const cell = state.board[row][column];
            if(cell.tilePlacement) {
                try {
                    const newRotation = mod(cell.tilePlacement.rotation + (isClockwise ? 1 : -1), 4)    // 4 different rotations
                    state.nile.rotate_tile(new Coordinates(row, column), newRotation);
                    dispatch({type: "rotateTile", coordinates: [row, column], rotation: newRotation});
                } catch (e) {
                    console.error(e);
                }
            }
        }
    }
    const onEndTurn = () => {
        try {
            const update = state.nile.end_turn();
            dispatch({type: "endTurn", turnScore: update.get_turn_score(), tiles: update.get_tiles()});
        } catch(e) {
            console.error(e);
        }
    }
    const onUndo = () => {
        try {
            const score = state.nile.undo();
            dispatch({type: "undo"});
        } catch (e) {
            console.error(e);
        }
    }
    const onRedo = () => {
        try {
            const score = state.nile.redo();
            dispatch({type: "redo"});
        } catch (e) {
            console.error(e);
        }
    }

    // Render
    return (
        <>
            <ul>
                { state.playerData.map((player, id) => (
                    <Player player={ player }
                        isCurrentTurn={ id === state.currentPlayerId }
                        setDraggedTile={ (idx, tile) => {
                            if (tile === Tile.Universal) {
                                dispatch({
                                    type: "setDraggedTile",
                                    isUniversal: true, tilePath: TilePath.Straight, idx
                                });
                            } else {
                                const tpt = TilePathType.tile_into_normal(tile);
                                dispatch({
                                    type: "setDraggedTile",
                                    isUniversal: false, tilePath: tpt.tile_path(), idx
                                });
                            }
                        } }
                    />
                )) }
            </ul>
            <div>
                <Button enabled={ state.selectedTile !== null }
                    onClick={ () => onRotate(false) }
                >
                    Rotate Counter-Clockwise
                </Button>
                <Button enabled={ state.selectedTile !== null }
                    onClick={ () => onRotate(true) }
                >
                    Rotate Clockwise
                </Button>
                <Button enabled={ state.nile.can_undo() }
                    onClick={ onUndo }
                >
                    Undo
                </Button>
                <Button enabled={ state.nile.can_redo() }
                    onClick={ onRedo }
                >
                    Redo
                </Button>
                <Button
                    // Must have played at least one tile
                    enabled={ state.currentTurnTiles.length > 0 }
                    onClick={ onEndTurn }
                >
                    End Turn
                </Button>
            </div>
            <Board board={ state.board }
                selectedTile={ state.selectedTile }
                onDropFromRack={ onDropFromRack }
                onSelect={ (coordinates) => dispatch({type: "selectTile", coordinates}) }
            />
        </>
    );
};
App.displayName = "App";

import { Board } from "components/Board";
import { Button } from "components/Button";
import { Player } from "components/Player";
import { useEventListener } from "lib/hooks";
import { initState, reducer } from "lib/state";
import { mod } from "lib/utils";
import { Coordinates, Rotation, Tile, TilePath, TilePathType } from "nile";
import React from "react";

export const Game: React.FC<{playerNames: string[]}> = ({playerNames}) => {
    // State
    const [fullState, dispatch] = React.useReducer(reducer, [], () => initState(playerNames));
    // Never want to mutate history
    const state = fullState.now;

    // Event handlers
    useEventListener("keydown", (e: KeyboardEvent) => {
        console.log(e.key)
        if (e.key === "q") {
            if (state.selectedTile) {
                onRotate(false);
            }
        } else if (e.key === "e") {
            if (state.selectedTile) {
                onRotate(true);
            }
        } else if (e.key === "x") {
            if (state.selectedTile) {
                // remove
            }
        }
    });
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
            const _score = state.nile.undo();
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
Game.displayName = "Game";

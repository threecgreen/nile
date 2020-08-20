import { Board } from "components/Board";
import { TileRack } from "components/TileRack";
import { Rotation, Tile as TileEnum, WasmNile, Coordinates } from "nile";
import React from "react";
import { Button } from "components/Button";
import { reducer, initState } from "lib/state";
import { useUndoReducer } from "lib/hooks";

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
                const score = state.nile.place_tile(state.draggedTile, new Coordinates(row, column), rotation);
                dispatch({type: "placeTile", tile: state.draggedTile, coordinates: [row, column], rotation, score});
            } catch (e) {
                console.error(e);
            }
        }
    }
    const onEndTurn = () => {}
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
                { state.playerData.map((player, id) => {
                    const playerName = players[id];
                    return (
                        <li key={ playerName }>
                            <h2>{ playerName }</h2>
                            <TileRack tiles={ player.tileRack }
                                isCurrentTurn={ id === state.currentPlayerId }
                                onDrag={ (tile) => dispatch({type: "setDraggedTile", tile}) }
                            />
                        </li>
                    );
                }) }
            </ul>
            <div>
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
                onDropFromRack={ onDropFromRack }
            />
        </>
    );
};
App.displayName = "App";

import { Board } from "components/Board";
import { Button } from "components/Button";
import { TileRack } from "components/TileRack";
import { useUndoReducer } from "lib/hooks";
import { initState, reducer } from "lib/state";
import { Coordinates, Rotation } from "nile";
import React from "react";

export const App: React.FC = () => {
    // State
    const players = ["player1", "player2"];
    const [state, dispatch] = useUndoReducer(reducer, () => initState(players))

    // Event handlers
    const onDropFromRack = (row: number, column: number) => {
        if (state.present.draggedTile !== null) {
            // Move this to another file
            try {
                const rotation = Rotation.None;
                console.log("PLacing tile");
                const score = state.present.nile.place_tile(state.present.draggedTile, new Coordinates(row, column), rotation);
                console.log("Received swasm response");
                dispatch({type: "placeTile", tile: state.present.draggedTile, coordinates: [row, column], rotation, score});
            } catch (e) {
                console.error(e);
            }
        }
    }
    const onEndTurn = () => {}
    const onUndo = () => {
        try {
            const score = state.present.nile.undo();
            dispatch({type: "undo"});
        } catch (e) {
            console.error(e);
        }
    }
    const onRedo = () => {
        try {
            const score = state.present.nile.redo();
            dispatch({type: "redo"});
        } catch (e) {
            console.error(e);
        }
    }

    // Render
    return (
        <>
            <ul>
                { state.present.playerData.map((player, id) => {
                    return (
                        <li key={ player.name }>
                            <h2>{ player.name }</h2>
                            <TileRack tiles={ player.tileRack }
                                isCurrentTurn={ id === state.present.currentPlayerId }
                                onDrag={ (tile) => dispatch({type: "setDraggedTile", tile}) }
                            />
                        </li>
                    );
                }) }
            </ul>
            <div>
                <Button enabled={ state.present.nile.can_undo() }
                    onClick={ onUndo }
                >
                    Undo
                </Button>
                <Button enabled={ state.present.nile.can_redo() }
                    onClick={ onRedo }
                >
                    Redo
                </Button>
                <Button
                    // Must have played at least one tile
                    enabled={ state.present.currentTurnTiles.length > 0 }
                    onClick={ onEndTurn }
                >
                    End Turn
                </Button>
            </div>
            <Board board={ state.present.board }
                onDropFromRack={ onDropFromRack }
            />
        </>
    );
};
App.displayName = "App";

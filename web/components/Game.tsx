import { Board } from "components/Board";
import { useEventListener } from "lib/hooks";
import { initState, reducer } from "lib/state";
import { mod, maxBy } from "lib/utils";
import { Coordinates, Rotation, TilePath, TilePathType } from "nile";
import React from "react";
import { Controls } from "./Controls";
import { Players } from "./Players";
import { sumTurnScores } from "lib/common";

export const Game: React.FC<{playerNames: string[], aiPlayerCount: number}> = ({playerNames, aiPlayerCount}) => {
    // State
    const [fullState, dispatch] = React.useReducer(reducer, [], () => initState(playerNames, aiPlayerCount));
    // Never want to mutate history
    const state = fullState.now;

    const logError = (e: Error) => {
        console.warn(`Error: ${e.message}; FullState: ${JSON.stringify(fullState)}`);
    }

    React.useEffect(() => {
        if (state.playerData[state.currentPlayerId].isCpu) {
            const cpuUpdate = state.nile.take_cpu_turn();
            if (cpuUpdate) {
                dispatch({type: "cpuTurn", cpuUpdate});
            }
        }
    }, [state.currentPlayerId]);

    // Event handlers
    useEventListener("keydown", (e: KeyboardEvent) => {
        // TODO: create help text for these
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
                onRemoveTile();
            }
        } else if (e.key === "u") {
            if (fullState.past.length > 0) {
                onUndo();
            }
        } else if (e.key === "r") {
            if (fullState.future.length > 0) {
                onRedo();
            }
        } else if (e.key === "E") {
            if (state.currentTurnTiles.length > 0) {
                onEndTurn();
            }
        } else if (e.key === "C") {
            if (state.currentTurnTiles.length === 0) {
                onCantPlay();
            }
        }
    });
    const onDrop = (row: number, column: number) => {
        if (state.draggedTile !== null) {
            // Move this to another file
            try {
                const rotation = Rotation.None;
                const tilePathType = state.draggedTile.isUniversal
                    ? TilePathType.universal(state.draggedTile.tilePath)
                    : TilePathType.normal(state.draggedTile.tilePath);
                const score = state.nile.place_tile(tilePathType, new Coordinates(row, column), rotation);
                dispatch({
                    type: "placeTile",
                    draggedTile: state.draggedTile, coordinates: [row, column],
                    rotation, score,
                });
            } catch (e) {
                logError(e);
            }
        // TODO: possibly separate this logic
        } else if (state.selectedTile !== null) {
            const oldCell = state.board[state.selectedTile[0]][state.selectedTile[1]];
            if (oldCell.tilePlacement) {
                try {
                    const tilePlacement = oldCell.tilePlacement;
                    const score = state.nile.move_tile(
                        new Coordinates(state.selectedTile[0], state.selectedTile[1]),
                        new Coordinates(row, column)
                    );
                    dispatch({
                        type: "moveTile",
                        oldCoordinates: state.selectedTile,
                        newCoordinates: [row, column],
                        tilePlacement,
                        score,
                    });
                } catch (e) {
                    logError(e);
                }
            } else {
                console.warn("Tried to move tile from cell with no tile");
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
                    logError(e);
                }
            }
        }
    }
    const onRemoveTile = () => {
        if(state.selectedTile) {
            const [row, column] = state.selectedTile;
            const cell = state.board[row][column];
            if(cell.tilePlacement) {
                try {
                    const score = state.nile.remove_tile(new Coordinates(row, column));
                    dispatch({type: "removeTile", coordinates: state.selectedTile, score});
                } catch (e) {
                    logError(e);
                }
            }
        }
    }
    const onUpdateUniversalPath = (tilePath: TilePath) => {
        if(state.selectedTile) {
            const [row, column] = state.selectedTile;
            const cell = state.board[row][column];
            if(cell.tilePlacement && cell.tilePlacement.isUniversal) {
                try {
                    state.nile.update_universal_path(new Coordinates(row, column), tilePath);
                    const tilePlacement = {...cell.tilePlacement, tilePath};
                    dispatch({type: "updateUniversalPath", coordinates: [row, column], tilePlacement});
                } catch (e) {
                    logError(e);
                }
            }
        }
    }
    const onEndTurn = () => {
        try {
            const update = state.nile.end_turn();
            dispatch({type: "endTurn", turnScore: update.turn_score, tiles: update.get_tiles(), hasEnded: update.game_has_ended});
        } catch(e) {
            alert(e);
        }
    }
    const onCantPlay = () => {
        try {
            const update = state.nile.cant_play();
            /// Save event as endTurn
            dispatch({type: "endTurn", turnScore: update.turn_score, tiles: update.get_tiles(), hasEnded: update.game_has_ended});
        } catch(e) {
            alert(e);
        }
    }
    const onUndo = () => {
        try {
            state.nile.undo();
            dispatch({type: "undo"});
        } catch (e) {
            logError(e);
        }
    }
    const onRedo = () => {
        try {
            state.nile.redo();
            dispatch({type: "redo"});
        } catch (e) {
            logError(e);
        }
    }

    const selectedIsUniversal = state.selectedTile !== null && (state.board[state.selectedTile[0]][state.selectedTile[1]].tilePlacement?.isUniversal ?? false);
    // Render
    return (
        <>
            <main>
                <h1>Nile</h1>
                { state.gameHasEnded && <h2>{ maxBy(state.playerData, (p) => sumTurnScores(p.scores))?.name } has won</h2>}
                {/* TODO: sticky header */}
                <Controls
                    hasPlacedTile={ state.currentTurnTiles.length > 0 }
                    hasSelectedTile={ state.selectedTile !== null }
                    selectedIsUniversal={ selectedIsUniversal }
                    canUndo={ fullState.past.length > 0 }
                    canRedo={ fullState.future.length > 0 }
                    onRotate={ onRotate }
                    onRemoveTile={ onRemoveTile }
                    onUpdateUniversalPath={ onUpdateUniversalPath }
                    onUndo={ onUndo }
                    onRedo={ onRedo }
                    onEndTurn={ onEndTurn }
                    onCantPlay={ onCantPlay }
                />
                <Board board={ state.board }
                    selectedTile={ state.selectedTile }
                    currentTurnTiles={ state.currentTurnTiles }
                    onDropFromRack={ onDrop }
                    onSelect={ (coordinates) => dispatch({type: "selectTile", coordinates}) }
                    // TODO: may want separate logic for this in the future
                    onDragStart={ (coordinates) => dispatch({type: "selectTile", coordinates}) }
                />
            </main>
            <footer>
                {/* TODO: sticky footer */}
                <Players currentPlayerId={ state.currentPlayerId }
                    playerData={ state.playerData }
                    setDraggedTile={ (isUniversal, tilePath, idx) => dispatch({type: "setDraggedTile", isUniversal, tilePath, idx}) }
                />
            </footer>
        </>
    );
};
Game.displayName = "Game";

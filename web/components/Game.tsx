import { Board } from "components/Board";
import { sumTurnScores } from "lib/common";
import { useEventListener } from "lib/hooks";
import { initState, reducer } from "lib/state";
import { maxBy, mod } from "lib/utils";
import { Coordinates, Rotation, Tile, TilePath, TilePathType } from "nile";
import React from "react";
import { Controls } from "./Controls";
import { Players } from "./Players";
import { Modal } from "./Modal";

export const Game: React.FC<{playerNames: string[], cpuPlayerCount: number}> = ({playerNames, cpuPlayerCount: aiPlayerCount}) => {
    // State
    const [fullState, dispatch] = React.useReducer(reducer, [], () => initState(playerNames, aiPlayerCount));
    // Never want to mutate history
    const state = fullState.now;

    React.useEffect(() => {
        if (state.playerData[state.currentPlayerId].isCpu) {
            const cpuUpdate = state.nile.take_cpu_turn();
            if (cpuUpdate) {
                dispatch({type: "cpuTurn", cpuUpdate});
            }
        }
    }, [state.currentPlayerId, state.nile, state.playerData]);

    // Event handlers
    useEventListener("keydown", (e: KeyboardEvent) => {
        if (e.ctrlKey || e.altKey || e.metaKey) {
            return;
        }
        // TODO: create help text for these
        switch (e.key) {
            case "q": {
                if (state.selectedTile) {
                    onRotate(false);
                }
                break;
            }
            case "x": {
                if (state.selectedTile) {
                    onRemoveTile();
                }
                break;
            }
            case "e": {
                if (state.selectedTile) {
                    onRotate(true);
                }
                break;
            }
            case "u": {
                if (fullState.past.length > 0) {
                    onUndo();
                }
                break;
            }
            case "r": {
                if (fullState.future.length > 0) {
                    onRedo();
                }
                break;
            }
            case "E": {
                if (state.currentTurnTiles.length > 0) {
                    onEndTurn();
                }
                break;
            }
            case "C": {
                if (state.currentTurnTiles.length === 0) {
                    onCantPlay();
                }
                break;
            }
            case "1":
            case "2":
            case "3":
            case "4":
            case "5": {
                const num = parseInt(e.key, 10);
                // Zero-indexed
                const idx = num - 1;
                const rack = state.playerData[state.currentPlayerId].tileRack;
                if (idx < rack.length) {
                    const tile = rack[idx];
                    const isUniversal = tile === Tile.Universal;
                    const tilePath = isUniversal ? TilePath.Straight : TilePathType.tile_into_normal(tile).tile_path();
                    dispatch({type: "selectRackTile", idx, isUniversal, tilePath});
                }
                break;
            }
        }
    });
    const onPlaceOnBoard = React.useCallback((row: number, column: number) => {
        if (state.selectedTile) {
            switch (state.selectedTile.type) {
                case "rack": {
                    try {
                        const rotation = Rotation.None;
                        const tile = {...state.selectedTile.tile};
                        const tilePathType = tile.isUniversal
                            ? TilePathType.universal(tile.tilePath)
                            : TilePathType.normal(tile.tilePath);
                        const score = state.nile.place_tile(tilePathType, new Coordinates(row, column), rotation);
                        dispatch({
                            type: "placeTile",
                            draggedTile: tile, coordinates: [row, column],
                            rotation, score,
                        });
                    } catch (e) {
                        dispatch({type: "setError", msg: e.message});
                    }
                    return;
                }
                case "board": {
                    const [prevRow, prevColumn] = state.selectedTile.coordinates;
                    const oldCell = state.board[prevRow][prevColumn];
                    if (oldCell.tilePlacement) {
                        try {
                            const tilePlacement = oldCell.tilePlacement;
                            const score = state.nile.move_tile(
                                new Coordinates(prevRow, prevColumn),
                                new Coordinates(row, column)
                            );
                            dispatch({
                                type: "moveTile",
                                oldCoordinates: [prevRow, prevColumn],
                                newCoordinates: [row, column],
                                tilePlacement,
                                score,
                            });
                        } catch (e) {
                            dispatch({type: "setError", msg: e.message});
                        }
                    } else {
                        dispatch({type: "setError", msg: "Tried to move tile from cell with no tile"});
                    }
                    return;
                }
            }
        }
    }, [state.board, state.nile, state.selectedTile]);
    const onRotate = React.useCallback((isClockwise: boolean) => {
        if(state.selectedTile?.type === "board") {
            const [row, column] = state.selectedTile.coordinates;
            const cell = state.board[row][column];
            if(cell.tilePlacement) {
                try {
                    const newRotation = mod(cell.tilePlacement.rotation + (isClockwise ? 1 : -1), 4)    // 4 different rotations
                    state.nile.rotate_tile(new Coordinates(row, column), newRotation);
                    dispatch({type: "rotateTile", coordinates: [row, column], rotation: newRotation});
                } catch (e) {
                    dispatch({type: "setError", msg: e.message});
                }
            }
        }
    }, [state.board, state.nile, state.selectedTile]);
    const onRemoveTile = React.useCallback(() => {
        if(state.selectedTile?.type === "board") {
            const [row, column] = state.selectedTile.coordinates;
            const cell = state.board[row][column];
            if(cell.tilePlacement) {
                try {
                    const score = state.nile.remove_tile(new Coordinates(row, column));
                    dispatch({type: "removeTile", coordinates: [row, column], score});
                } catch (e) {
                    dispatch({type: "setError", msg: e.message});
                }
            }
        }
    }, [state.board, state.nile, state.selectedTile]);
    const onUpdateUniversalPath = (tilePath: TilePath) => {
        if(state.selectedTile?.type === "board") {
            const [row, column] = state.selectedTile.coordinates;
            const cell = state.board[row][column];
            if(cell.tilePlacement && cell.tilePlacement.isUniversal) {
                try {
                    state.nile.update_universal_path(new Coordinates(row, column), tilePath);
                    const tilePlacement = {...cell.tilePlacement, tilePath};
                    dispatch({type: "updateUniversalPath", coordinates: [row, column], tilePlacement});
                } catch (e) {
                    dispatch({type: "setError", msg: e.message});
                }
            }
        }
    }
    const onEndTurn = React.useCallback(() => {
        try {
            const update = state.nile.end_turn();
            dispatch({type: "endTurn", turnScore: update.turn_score, tiles: update.get_tiles(), hasEnded: update.game_has_ended});
        } catch(e) {
            dispatch({type: "setError", msg: e.message});
        }
    }, [state.nile]);
    const onCantPlay = React.useCallback(() => {
        try {
            const update = state.nile.cant_play();
            /// Save event as endTurn
            dispatch({type: "endTurn", turnScore: update.turn_score, tiles: update.get_tiles(), hasEnded: update.game_has_ended});
        } catch(e) {
            dispatch({type: "setError", msg: e.message});
        }
    }, [state.nile]);
    const onUndo = React.useCallback(() => {
        try {
            state.nile.undo();
            dispatch({type: "undo"});
        } catch (e) {
            console.warn(`Error: ${e.message};`);
        }
    }, [state.nile]);
    const onRedo = React.useCallback(() => {
        try {
            state.nile.redo();
            dispatch({type: "redo"});
        } catch (e) {
            console.warn(`Error: ${e.message};`);
        }
    }, [state.nile]);

    const selectedIsUniversal = state.selectedTile?.type === "board"
        && (state.board[state.selectedTile.coordinates[0]][state.selectedTile.coordinates[1]].tilePlacement?.isUniversal ?? false);
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
                    selectedTile={ state.selectedTile?.type === "board" ? state.selectedTile.coordinates : null }
                    currentTurnTiles={ state.currentTurnTiles }
                    onDropFromRack={ onPlaceOnBoard }
                    onSelect={ (coordinates) => dispatch({type: "selectBoardTile", coordinates}) }
                    // TODO: may want separate logic for this in the future
                    onDragStart={ (coordinates) => dispatch({type: "selectBoardTile", coordinates}) }
                />
                { state.modal && <Modal>{ state.modal.msg }</Modal> }
            </main>
            <footer>
                {/* TODO: sticky footer */}
                <Players currentPlayerId={ state.currentPlayerId }
                    selectedTileIdx={ state.selectedTile?.type === "rack" ? state.selectedTile.tile.idx : null }
                    playerData={ state.playerData }
                    onSelect={ (isUniversal, tilePath, idx) => dispatch({type: "selectRackTile", isUniversal, tilePath, idx}) }
                />
            </footer>
        </>
    );
};
Game.displayName = "Game";

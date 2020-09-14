import { Board } from "components/Board";
import { sumTurnScores } from "lib/common";
import { useEventListener, useRefFun } from "lib/hooks";
import { StateManager } from "lib/state";
import { maxBy } from "lib/utils";
import { WasmNile } from "nile";
import React from "react";
import { Controls } from "./Controls";
import { Modal } from "./Modal";
import { Players } from "./Players";

export const Game: React.FC<{playerNames: string[], cpuPlayerCount: number}> = ({playerNames, cpuPlayerCount: aiPlayerCount}) => {
    // State
    const nileRef = useRefFun(() => new WasmNile(playerNames, aiPlayerCount));
    const stateManagerRef = useRefFun(() => new StateManager(nileRef.current, React.useReducer));
    const stateManager = stateManagerRef.current;
    const state = stateManager.state;

    /// Even though `stateManager.takeCpuTurn` takes no arguments, we need to define a closure
    /// to maintain up to date references and not break eslint's exhaustive dependency check
    React.useEffect(() => stateManager.takeCpuTurn(), [stateManager, state.currentPlayerId]);

    // Event handlers
    useEventListener("keydown", (e: KeyboardEvent) => {
        if (e.ctrlKey || e.altKey || e.metaKey) {
            return;
        }
        // TODO: create help text for these
        switch (e.key) {
            case "q": {
                if (state.selectedTile) {
                    stateManager.rotate(false);
                }
                break;
            }
            case "e": {
                if (state.selectedTile) {
                    stateManager.rotate(true);
                }
                break;
            }
            case "x": {
                if (state.selectedTile) {
                    stateManager.removeSelectedTile();
                }
                break;
            }
            case "u": {
                if (stateManager.canUndo) {
                    stateManager.undo();
                }
                break;
            }
            case "r": {
                if (stateManager.canRedo) {
                    stateManager.redo();
                }
                break;
            }
            case "E": {
                if (state.currentTurnTiles.length > 0) {
                    stateManager.endTurn();
                }
                break;
            }
            case "C": {
                if (state.currentTurnTiles.length === 0) {
                    stateManager.cantPlay();
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
                stateManager.selectRackTile(idx);
                break;
            }
        }
    });

    const placeOnBoard = React.useCallback((row, col) => stateManager.placeOnBoard(row, col), [stateManager]);
    const selectBoardTile = React.useCallback((coordinates) => stateManager.selectBoardTile(coordinates), [stateManager]);

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
                    canUndo={ stateManager.canUndo }
                    canRedo={ stateManager.canRedo }
                    onRotate={ stateManager.rotate }
                    onRemoveTile={ stateManager.removeSelectedTile }
                    onUpdateUniversalPath={ stateManager.updateUniversalPath }
                    onUndo={ () => stateManager.undo() }
                    onRedo={ () => stateManager.redo() }
                    onEndTurn={ stateManager.endTurn }
                    onCantPlay={ stateManager.cantPlay }
                />
                <Board board={ state.board }
                    selectedTile={ state.selectedTile?.type === "board" ? state.selectedTile.coordinates : null }
                    currentTurnTiles={ state.currentTurnTiles }
                    onDropFromRack={ placeOnBoard }
                    onSelect={ selectBoardTile }
                    onDragStart={ selectBoardTile }
                />
                { state.modal && <Modal>{ state.modal.msg }</Modal> }
            </main>
            <footer>
                {/* TODO: sticky footer */}
                <Players currentPlayerId={ state.currentPlayerId }
                    selectedTileIdx={ state.selectedTile?.type === "rack" ? state.selectedTile.tile.idx : null }
                    playerData={ state.playerData }
                    onSelect={ (idx) => stateManager.selectRackTile(idx) }
                />
            </footer>
        </>
    );
};
Game.displayName = "Game";

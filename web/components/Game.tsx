import { Board } from "components/Board";
import { useEventListener, useRefFun } from "lib/hooks";
import { StateManager } from "lib/state";
import { WasmNile } from "nile";
import React from "react";
import { Controls } from "./Controls";
import { ErrorModal } from "./Modal";
import { Players } from "./Players";

interface IProps {
    playerNames: string[];
    cpuPlayerCount: number;
}

export const Game: React.FC<IProps> = ({playerNames, cpuPlayerCount}) => {
    // State
    const nileRef = useRefFun(() => new WasmNile(playerNames, cpuPlayerCount));
    const [state, stateManager] = StateManager.useStateManager(nileRef.current);

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
                if (state.currentTurnTiles.size > 0) {
                    stateManager.endTurn();
                }
                break;
            }
            case "C": {
                if (state.currentTurnTiles.size === 0) {
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

    // Render
    return (
        <>
            <main>
                {/* TODO: sticky header */}
                <Controls
                    hasPlacedTile={ state.currentTurnTiles.size > 0 }
                    hasSelectedTile={ state.selectedTile !== null }
                    selectedIsUniversal={ stateManager.selectedIsUniversal }
                    canUndo={ stateManager.canUndo }
                    canRedo={ stateManager.canRedo }
                    onRotate={ (isClockwise) => stateManager.rotate(isClockwise) }
                    onRemoveTile={ () => stateManager.removeSelectedTile() }
                    onUpdateUniversalPath={ (tp) => stateManager.updateUniversalPath(tp) }
                    onUndo={ () => stateManager.undo() }
                    onRedo={ () => stateManager.redo() }
                    onEndTurn={ () => stateManager.endTurn() }
                    onCantPlay={ () => stateManager.cantPlay() }
                />
                <Board board={ state.board }
                    selectedTile={ state.selectedTile?.type === "board" ? state.selectedTile.coordinates : null }
                    currentTurnTiles={ state.currentTurnTiles }
                    onDropFromRack={ (r, c) => stateManager.placeOnBoard(r, c) }
                    onSelect={ (c) => stateManager.selectBoardTile(c) }
                />
                <ErrorModal msg={ state.modal?.msg ?? null }
                    dismiss={ () => stateManager.dismiss() }
                />
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

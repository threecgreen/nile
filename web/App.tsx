import { Board } from "components/Board";
import { TileRack } from "components/TileRack";
import { PlaceTileEvent, UndoEvent } from "lib/events";
import { Rotation, Tile as TileEnum, WasmNile, Coordinates } from "nile";
import React from "react";
import { Button } from "components/Button";

export const App: React.FC = () => {
    // Wasm state
    const players = ["player1", "player2"];
    const [nile, _] = React.useState(() => new WasmNile(players));
    // TODO: probably want separate implementation of board state for UI
    const [board, setBoard] = React.useState(() => nile.board());
    const [playerData, setPlayerData] = React.useState(() => nile.players());
    const width = board.width();
    const height = board.height();
    // Other state
    const [currentTurnPlayerId, setCurrentTurnPlayerId] = React.useState<number>(0);
    const [draggedTile, setDraggedTile] = React.useState<TileEnum | null>(null);
    /// Used for determining if placed tile is movable, rotatable, etc.
    const [currendTurnTiles, setCurrentTurnTiles] = React.useState<Array<[number, number]>>([]);

    // Event handlers
    const onDropFromRack = (row: number, column: number) => {
        if (draggedTile !== null) {
            // Move this to another file
            try {
                const score = nile.place_tile(draggedTile, new Coordinates(row, column), Rotation.None);
                setDraggedTile(null);
                setCurrentTurnTiles((tiles) =>
                    [...tiles, [row, column]]
                );
                const updatedBoard = nile.board();
                console.log(updatedBoard === board);
                setBoard(updatedBoard);
                setPlayerData(nile.players());
            } catch (e) {
                console.error(e);
            }
        }
    }
    const onEndTurn = () => {}
    const onUndo = () => {
        try {
            const score = nile.undo();
            setBoard(nile.board());;
            setPlayerData(nile.players());
        } catch (e) {
            console.error(e);
        }
    }
    const onRedo = () => {}

    // Render
    return (
        <>
            <ul>
                { playerData.map((player, id) => {
                    const playerName = players[id];
                    const tiles: string[] = player.get_tiles();
                    return (
                        <li key={ playerName }>
                            <h2>{ playerName }</h2>
                            <TileRack tiles={ tiles.map((t) =>
                                // @ts-ignore
                                TileEnum[t as keyof TileEnum]) }
                                isCurrentTurn={ id === currentTurnPlayerId }
                                onDrag={ setDraggedTile }
                            />
                        </li>
                    );
                }) }
            </ul>
            <div>
                <Button enabled={ nile.can_undo() }
                    onClick={ onUndo }
                >
                    Undo
                </Button>
                <Button enabled={ nile.can_redo() }
                    onClick={ onRedo }
                >
                    Redo
                </Button>
                <Button
                    // Must have played at least one tile
                    enabled={ currendTurnTiles.length > 0 }
                    onClick={ onEndTurn }
                >
                    End Turn
                </Button>
            </div>
            <Board height={ height }
                width={ width }
                board={ board }
                onDropFromRack={ onDropFromRack }
            />
        </>
    );
};
App.displayName = "App";

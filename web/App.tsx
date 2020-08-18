import { Board } from "components/Board";
import { Tile } from "components/Tile";
import { Player, Rotation, Tile as TileEnum, WasmNile, Coordinates } from "nile";
import React from "react";
import { TileRack } from "components/TileRack";

export const App: React.FC = () => {
    // Wasm state
    const players = ["player1", "player2"];
    const [nile, _] = React.useState(() => new WasmNile(players));
    const [board, setBoard] = React.useState(() => nile.board());
    const [playerData, setPlayerData] = React.useState(() => nile.players());
    const width = board.width();
    const height = board.height();
    // Other state
    const [currentTurnPlayerId, setCurrentTurnPlayerId] = React.useState<number>(0);
    const [draggedTile, setDraggedTile] = React.useState<TileEnum | null>(null);

    // Event handlers
    const onDrop = (row: number, column: number) => {
        if (draggedTile) {
            // Move this to another file
            try {
                nile.handle_event({ PlaceTile: { tile: draggedTile, coordinates: new Coordinates(row, column), rotation: Rotation.None }});
                setBoard(nile.board());
                setPlayerData(nile.players());
            } catch (e) {
                console.error(e);
            }
        }
    }

    // Render
    return (
        <>
            <ul>
                {playerData.map((player, id) => {
                    const playerName = players[id];
                    const tiles: string[] = player.get_tiles();
                    return (
                        <li key={ playerName }>
                            <h2>{ playerName }</h2>
                            <TileRack tiles={tiles.map((t) =>
                                // @ts-ignore
                                TileEnum[t as keyof TileEnum])}
                                isCurrentTurn={id === currentTurnPlayerId}
                                onDrag={setDraggedTile}
                            />
                        </li>
                    );
                })}
            </ul>
            <Board height={height}
                width={width}
                board={board}
                onDrop={onDrop}
            />
        </>
    );
};
App.displayName = "App";

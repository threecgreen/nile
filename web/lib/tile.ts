import { Rotation, Tile } from "nile";

const TILE_SIZE = 40;   // px
const GRID_LINE_WIDTH = 1;  // px

export const drawTilePlacement = (ctx: CanvasRenderingContext2D, row: number, column: number, tile: Tile, _rotation: Rotation) => {
    const topLeft: [number, number] = [row * (TILE_SIZE + GRID_LINE_WIDTH), column * (TILE_SIZE + GRID_LINE_WIDTH)];
    switch (tile) {
        case Tile.Straight:
            ctx.moveTo(topLeft[0], topLeft[1] + TILE_SIZE / 2);
            ctx.lineTo(topLeft[0] + TILE_SIZE, topLeft[1] + TILE_SIZE / 2);
            return;
        case Tile.Diagonal:
            ctx.moveTo(topLeft[0], topLeft[1] + TILE_SIZE);
            ctx.lineTo(topLeft[0] + TILE_SIZE, topLeft[1]);
            return;
        default:
            console.error("Unknown tile type");
    }
}

import { Rotation, Tile } from "nile";
import { drawTilePlacement } from "./lib/tile";

const GRID_COLOR = "#0f0f0f";

const canvas = document.getElementById("nile-canvas") as HTMLCanvasElement;
const ctx = canvas.getContext('2d');
if (ctx) {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    drawTilePlacement(ctx, 0, 0, Tile.Straight, Rotation.None);
    drawTilePlacement(ctx, 0, 1, Tile.Diagonal, Rotation.None);

    ctx.stroke();
}

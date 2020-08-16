import { render } from "react-dom";
import { App } from "./App";
import { createElement } from "react";

const GRID_COLOR = "#0f0f0f";

const container = document.getElementById("app-container") as HTMLDivElement;
render(createElement(App), container);

// if (ctx) {
//     ctx.beginPath();
//     ctx.strokeStyle = GRID_COLOR;

//     drawTilePlacement(ctx, 0, 0, Tile.Straight, Rotation.None);
//     drawTilePlacement(ctx, 0, 1, Tile.Diagonal, Rotation.None);

//     ctx.stroke();
// }

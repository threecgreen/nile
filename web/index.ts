import { createElement } from "react";
import { render } from "react-dom";
import { App } from "./App";

const GRID_COLOR = "#0f0f0f";

const container = document.getElementById("app-container") as HTMLDivElement;
render(createElement(App), container);

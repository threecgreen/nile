import { createElement } from "react";
import { render } from "react-dom";
import { App } from "./front_end/App";

const container = document.getElementById("app-container") as HTMLDivElement;
render(createElement(App), container);

import React from "react";
import { initState, reducer } from "../state";
import { act, renderHook } from '@testing-library/react-hooks';

// // jest.mock("nile", () => ({
// // //   __esModule: true, // this property makes it work
// //   WasmNile: jest.fn(),
// // }));
// // import { WasmNile } from "nile";

// const mockNile = WasmNile as unknown as jest.Mock<typeof WasmNile>;

// const setup = () => {
//     const { result } = renderHook(() => React.useReducer(reducer, ["test1", "test2"], initState));
//     return result;
// }

// test("Moving tile has no net price effect", () => {

// });

// test("Moving tile updates currentTurnTiles", () => {

// });

// test("Placing tile nullifies draggedTile", () => {

// });

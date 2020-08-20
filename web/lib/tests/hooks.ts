import React from "react";
import { act, renderHook } from '@testing-library/react-hooks';
import { useUndoReducer } from "lib/hooks";

type TestState = {
    nums: number[];
}

type Action =
    | {type: "undo"}
    | {type: "redo"}
    | {type: "add", num: number}

const reducer: React.Reducer<TestState, Action> = (state, action) => {
    switch (action.type) {
        case "add":
            const nums = [...state.nums];
            nums.push(action.num);
            return {nums};
        default:
            return state;
    }
}

test("Can undo", () => {
    const { result } = renderHook(() => useUndoReducer(reducer, {nums: [0]}));
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "add", num: 1});
        dispatch({type: "add", num: 2});
        dispatch({type: "add", num: 3});
    });
    expect(result.current[0].nums).toEqual([0, 1, 2, 3]);
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "undo"});
        dispatch({type: "undo"});
    });
    expect(result.current[0].nums).toEqual([0, 1]);
});

test("Can redo", () => {
    const { result } = renderHook(() => useUndoReducer(reducer, {nums: [0]}));
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "add", num: 2});
        dispatch({type: "add", num: 4});
        dispatch({type: "add", num: 6});
        dispatch({type: "add", num: 8});
    });
    expect(result.current[0].nums).toEqual([0, 2, 4, 6, 8]);
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "undo"});
        dispatch({type: "undo"});
    });
    expect(result.current[0].nums).toEqual([0, 2, 4]);
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "redo"});
        dispatch({type: "redo"});
    });
    expect(result.current[0].nums).toEqual([0, 2, 4, 6, 8]);
})

test("Invalid undo is no-op", () => {
    const nums = [5, 9];
    const { result } = renderHook(() => useUndoReducer(reducer, {nums}));
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "undo"});
        dispatch({type: "undo"});
    });
    expect(result.current[0].nums).toBe(nums);
})

test("Invalid undo is no-op", () => {
    const nums = [5, 9];
    const { result } = renderHook(() => useUndoReducer(reducer, {nums}));
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "add", num: 7});
        dispatch({type: "add", num: 9});
    });
    const allNums = result.current[0].nums;
    expect(allNums.length).toBe(4);
    act(() => {
        const [_, dispatch] = result.current;
        dispatch({type: "redo"});
        dispatch({type: "redo"});
    });
    expect(allNums.length).toBe(4);
})

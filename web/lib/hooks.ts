import React, { Reducer, ReducerState, Dispatch, ReducerAction } from "react";

interface IState<S> {
    past: S[];
    present: S;
    future: S[];
}

// Credit to @johanquiroga https://github.com/johanquiroga
export function useUndoReducer<R extends Reducer<any, any>>(
    reducer: R,
    initState: ReducerState<R> | (() => ReducerState<R>)
): [ReducerState<R>, Dispatch<ReducerAction<R>>] {
    type IUndoState = IState<ReducerState<R>>;
    const undoState: IUndoState = {
        past: [],
        present: initState instanceof Function ? initState() : initState,
        future: [],
    };

    const undoReducer: Reducer<IUndoState, ReducerAction<R>> = (state, action) => {
        if (action.type === "undo") {
            if (state.past.length > 0) {
                const [newPresent, ...past] = state.past;
                return {
                    past,
                    present: newPresent,
                    future: [state.present, ...state.future]
                };
            }
            return state;
        }
        if (action.type === "redo") {
            if (state.future.length > 0) {
                const [newPresent, ...future] = state.future;
                return {
                    past: [state.present, ...state.past],
                    present: newPresent,
                    future
                };
            }
            return state;
        }
        const newPresent = reducer(state.present, action);
        return {
            past: [state.present, ...state.past],
            present: newPresent,
            future: []
        };
    };

    const [state, dispatch] = React.useReducer(undoReducer, undoState);
    return [state.present, dispatch];
};

export type UndoAction =
    | {type: "undo"}
    | {type: "redo"};

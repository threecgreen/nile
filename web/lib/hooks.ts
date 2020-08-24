import React, { Reducer, ReducerState, Dispatch, ReducerAction } from "react";

interface IState<S> {
    past: S[];
    present: S;
    future: S[];
}

// Credit to @johanquiroga https://github.com/johanquiroga
// TODO: this probably needs to be integrated with state.ts to clear past and
// future at the end of each turn and skip some events like selection
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

export const useEventListener = (
    eventName: string,
    handler: (event: any) => void,
    element = window
) => {
    // Create a ref that stores handler
    const savedHandler = React.useRef<(event: any) => void>();

    // Update ref.current value if handler changes.
    // ... without us needing to pass it in effect deps array ...
    // ... and potentially cause effect to re-run every render.
    React.useEffect(() => {
        savedHandler.current = handler;
    }, [handler]);

    React.useEffect(
        () => {
            // Make sure element supports addEventListener
            // On
            const isSupported = element && element.addEventListener;
            if (!isSupported) {
                return;
            }

            // Create event listener that calls handler function stored in ref
            const eventListener: EventListener = (event) => {
                if (savedHandler.current) {
                    savedHandler.current(event);
                }
            }

            // Add event listener
            element.addEventListener(eventName, eventListener);

            // Remove event listener on cleanup
            return () => {
                element.removeEventListener(eventName, eventListener);
            };
        },
        [eventName, element] // Re-run if eventName or element changes
    );
};

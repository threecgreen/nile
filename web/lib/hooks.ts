/* eslint-disable @typescript-eslint/no-explicit-any */
import React from "react";

export const useEventListener = (
    eventName: string,
    handler: (event: any) => void,
    element = window
): void => {
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

/** Like @c React.useRef , but supports an initializer function. This is useful for objects that
 * are expensive to initialize, e.g. a Wasm object */
export const useRefFun = <T>(init: () => T): React.MutableRefObject<T> => {
    const ref = React.useRef<T | null>(null);
    if (ref.current === null) {
        ref.current = init();
    }
    return ref as React.MutableRefObject<T>;
}

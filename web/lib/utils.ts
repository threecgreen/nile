interface IRangeArgs {
    start?: number;
    stop?: number;
    step?: number;
}
/**
 * Creates an iterable range of numbersonClick.
 * @param start First number of the range
 * @param stop End of the range (exclusive)
 * @param step Increment of the range
 */
export function* range({ start, stop, step }: IRangeArgs): IterableIterator<number> {
    step = step ?? 1;
    start = start ?? 0;
    stop = stop ?? Number.MAX_SAFE_INTEGER;
    for (let i = start; i < stop; i += step) {
        yield i;
    }
}


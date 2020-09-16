import { CoordinateTuple } from "./common";

/** Implements an interface similar to `Set`, but is meant for use in React
 * state and so any modifications will invalidate referencial equality by
 * returning a shallow copy with the modifications.
 *
 * Coordinates are modeled as a `[number, number]`, but arrays aren't hashed
 * by value in javascript
 */
export class CoordinateSet {
    private width: number;
    private set: Record<number, undefined>;

    public constructor(width: number) {
        this.width = width;
        this.set = {};
    }

    public add(coordinates: CoordinateTuple): CoordinateSet {
        const hash = this.hashCoordinates(coordinates);
        const copy = Object.create(this);
        copy.set = {...this.set, [hash]: undefined};
        return copy;
    }

    public clear(): CoordinateSet {
        const copy = Object.create(this);
        copy.set = {};
        return copy;
    }

    public delete(coordinates: CoordinateTuple): CoordinateSet {
        const hash = this.hashCoordinates(coordinates);
        const copy = Object.create(this);
        const set = {...this.set};
        delete set[hash];
        copy.set = set;
        return copy;
    }

    public has(coordinates: CoordinateTuple): boolean {
        const hash = this.hashCoordinates(coordinates);
        return hash in this.set;
    }

    public get size(): number {
        return Object.keys(this.set).length;
    }

    private hashCoordinates(coordinates: CoordinateTuple): number {
        const [row, column] = coordinates;
        return (row * this.width) + column;
    }
}

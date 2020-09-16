export class Timer {
    private readonly name: string;
    private startTime: number;
    private lastSplit: number;
    private splitTimes: Map<string, number>;

    public constructor(name: string) {
        this.name = name;
        this.startTime = performance.now();
        this.lastSplit = this.startTime;
        this.splitTimes = new Map();
    }

    /**
     * Create a new named split and returns the duration of the split
     * @param name
     */
    public split(name: string): number {
        const now = performance.now();
        const elapsed = now - this.lastSplit;
        this.splitTimes.set(name, elapsed);
        this.lastSplit = now;
        return elapsed;
    }

    public get splits(): Map<string, number> {
        return this.splitTimes;
    }

    public log(): void {
        const splitStr = Array.from(this.splitTimes.entries()).map(([k, v]) => `${k}: ${v}`).join(", ");
        console.log(`${this.name}: {${splitStr}}`);
    }

    /**
     * @returns milliseconds since start time
     */
    public totalElapsed(): number {
        return performance.now() - this.startTime;
    }
}

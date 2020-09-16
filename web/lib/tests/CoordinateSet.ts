import { CoordinateSet } from "lib/CoordinateSet";

test("add returns copy", () => {
    const target = new CoordinateSet(21);
    const modified = target.add([14, 20]);
    expect(target).toBe(target);
    expect(target).not.toBe(modified);
    expect(target.size).toEqual(0);
    expect(modified.size).toEqual(1);
});

test("add and has", () => {
    let target = new CoordinateSet(21);
    target = target.add([10, 8]).add([9, 0]);
    expect(target.has([11, 11])).toBeFalsy();
    const modified = target.add([11, 11]);
    expect(target.has([11, 11])).toBeFalsy();
    expect(modified.has([11, 11])).toBeTruthy();
});

test("clear", () => {
    const target = (new CoordinateSet(21))
        .add([5, 6])
        .add([7, 8]);
    expect(target.size).toEqual(2);
    const modified = target.clear();
    expect(target.size).toEqual(2);
    expect(modified.size).toEqual(0);
});

test("delete", () => {
    const target = (new CoordinateSet(21)).add([5, 6]);
    expect(target.has([5, 6])).toBeTruthy();
    expect(target.size).toEqual(1);
    const modified = target.delete([5, 6]);
    expect(modified.has([5, 6])).toBeFalsy();
    expect(modified.size).toEqual(0);
});

test("delete non-existent coordinates", () => {
    const target = (new CoordinateSet(21)).add([5, 4]).add([9, 9]).add([4, 0]);
    const modified = target.delete([10, 0]);
    expect(modified.size).toEqual(target.size);
});

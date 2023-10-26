
export const clamp = (value: number, min: number, max: number): number => {
    return Math.min(Math.max(value, min), max);
}

export const singleSnap = (value: number, target: number, deadRatio: number): number => {
    if (Math.abs(value - target) / target < deadRatio) {
        return target;
    }
    return value;
}
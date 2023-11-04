
export const replaceAtIndex = <T>(array: T[], index: number, value: T): T[] => {
    const newArray = [...array];
    newArray[index] = value;
    return newArray;
};

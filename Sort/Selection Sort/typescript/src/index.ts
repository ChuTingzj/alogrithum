#! /usr/bin/env ts-node
type TValuable<T extends number> = [T, ...T[]]

function SelectionSort<T extends number>(arr: TValuable<T>): TValuable<T> {
    for (let i = 0; i < arr.length; i++) {
        let min = arr[i];
        let minIndex = i;
        for (let j = i + 1; j < arr.length; j++) {
            if (arr[j] < min) {
                min = arr[j]
                minIndex = j
            }
        }
        let temp = arr[i]
        arr[i] = arr[minIndex]
        arr[minIndex] = temp
    }
    return arr
}

console.log(SelectionSort([29, 10, 14, 37, 14]))
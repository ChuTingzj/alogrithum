#! /usr/bin/env ts-node
type TValuable<T extends number> = [T, ...T[]]
const BubbleSort = <T extends number>(arr: TValuable<T>) => {
    for (let len = arr.length; len > 0; len--) {
        for (let i = 0; i < len - 1; i++) {
            if (arr[i] > arr[i + 1]) {
                let temp = arr[i + 1]
                arr[i + 1] = arr[i]
                arr[i] = temp
            }
        }
    }
    return arr
}

const flag = performance.now()
//0.003
console.log(BubbleSort([29, 10, 14, 37, 14]))
console.warn((performance.now()-flag)/1000)
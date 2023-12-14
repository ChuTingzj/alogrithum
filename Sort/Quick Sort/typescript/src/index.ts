#! /usr/bin/env ts-node
type TValuable<T extends number> = [T, ...T[]]

function QuickSort<T extends number>(arr: Array<T>): Array<T> {
  if (arr.length <= 1) {
    return arr;
  }

  const pivot = arr[0];
  const left = [];
  const right = [];

  for (let i = 1; i < arr.length; i++) {
    if (arr[i] < pivot) {
      left.push(arr[i]);
    } else {
      right.push(arr[i]);
    }
  }

  return QuickSort(left).concat(pivot, QuickSort(right));
}

console.log(QuickSort([29, 10, 14, 37, 14]))
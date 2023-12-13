#! /usr/bin/env ts-node
type TValuable<T extends number> = [T, ...T[]]

function Merge<T extends number>(arr1: Array<T>, arr2: Array<T>): Array<T> {
  const result = new Array<T>();
  while (arr1.length && arr2.length) {
    if (arr1[0] <= arr2[0]) {
      result.push(arr1[0])
      arr1.shift()
    } else {
      result.push(arr2[0])
      arr2.shift()
    }
  }
  if (arr1.length) {
    result.push(...arr1)
  }
  if (arr2.length) {
    result.push(...arr2)
  }
  return result
}

function MergeSort<T extends number>(arr: Array<T>): Array<T> {
  if (arr.length <= 1) return arr
  const mid = Math.ceil(arr.length / 2)
  let left = arr.slice(0, mid)
  let right = arr.slice(mid)
  left = MergeSort(left);
  right = MergeSort(right);
  return Merge(left, right)
}

console.log(MergeSort([29, 10, 14, 37, 14]))
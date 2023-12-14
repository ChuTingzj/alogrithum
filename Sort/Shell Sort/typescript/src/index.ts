#! /usr/bin/env ts-node
type TValuable<T extends number> = [T, ...T[]]

function ShellSort<T extends number>(arr:Array<T>):Array<T> {
  const len = arr.length;

  // 使用 Knuth 序列计算初始增量
  let h = 1;
  while (h < len / 3) {
    h = 3 * h + 1;
  }

  // 增量逐渐减小，直到为1
  while (h >= 1) {
    // 对每个子序列进行插入排序
    for (let i = h; i < len; i++) {
      // 插入排序
      for (let j = i; j >= h && arr[j] < arr[j - h]; j -= h) {
        // 交换元素
        [arr[j], arr[j - h]] = [arr[j - h], arr[j]];
      }
    }

    // 减小增量
    h = Math.floor(h / 3);
  }

  return arr;
}

console.log(ShellSort([29, 10, 14, 37, 14]))
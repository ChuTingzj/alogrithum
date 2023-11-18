#! /usr/bin/env ts-node
type TValuable<T extends number> = [T, ...T[]]

function InsertionSort<T extends number>(arr: TValuable<T>): TValuable<T> {
    for (let i = 1; i < arr.length; i++) {
        //未排序的第一个
        const currentVal = arr[i];
        //已排序的最后一个(最大的)
        let j = i - 1;
        //和已排序的比较
        //1、j<0说明currentVal在已排序的之中是最小的
        //2、arr[j] < currentVal说明currentVal在已排序之中是最大的
        while (j >= 0 && arr[j] > currentVal) {
            //最大值后移
            arr[j + 1] = arr[j];
            //更新尾索引
            j--;
        }
        //currentVal是较大值或最小值
        arr[j + 1] = currentVal;
    }
    return arr;
}

console.log(InsertionSort([29, 10, 14, 37, 14]))
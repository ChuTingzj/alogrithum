# 归并排序
归并排序是一种基于分治策略的经典排序算法，其核心思想是将一个大问题分解成多个小问题，解决小问题后再将结果合并得到整体解决方案。具体步骤如下：

1. 分解（Divide）：将待排序的数组递归地分解成两个子数组，直到每个子数组只有一个元素或为空。

2. 解决（Conquer）：对每个子数组进行排序。通常使用递归调用归并排序来解决子问题。

3. 合并（Combine）：将排好序的子数组合并成一个有序的数组。这一步是归并排序的关键，需要额外的辅助数组来暂存合并的结果。

整个过程的关键在于合并操作，它需要比较两个子数组的元素，并按顺序将它们合并到一个辅助数组中，最后再将辅助数组的内容复制回原数组。

归并排序的时间复杂度是 O(n log n)，其中 n 是待排序数组的大小。空间复杂度通常是 O(n)。尽管它的时间复杂度相对较低，但归并排序的缺点是需要额外的空间来存储辅助数组，这可能在处理大型数据集时成为一个不利之处。

下面是一个简单的归并排序的伪代码：

```plaintext
MergeSort(array):
    if length of array <= 1:
        return array
    
    // 分解
    middle = length of array / 2
    left_half = array[0, middle-1]
    right_half = array[middle, end]
    
    // 递归解决
    left_half = MergeSort(left_half)
    right_half = MergeSort(right_half)
    
    // 合并
    array = Merge(left_half, right_half)
    
    return array

Merge(left, right):
    result = []
    while left is not empty and right is not empty:
        if left[0] <= right[0]:
            append left[0] to result
            remove left[0] from left
        else:
            append right[0] to result
            remove right[0] from right
    
    // 处理剩余元素
    if left is not empty:
        append remaining elements of left to result
    if right is not empty:
        append remaining elements of right to result
    
    return result
```
# 快速排序
快速排序是一种常见的排序算法，它通过将一个数组分成较小的子数组来工作，然后递归地对这些子数组进行排序。算法的基本思想是选择一个基准元素，然后将数组中小于基准元素的值放在基准元素的左边，大于基准元素的值放在右边。这样，基准元素就位于最终排序数组的正确位置上。

快速排序的步骤如下：

+ 选择一个基准元素。
+ 将数组分割成两个子数组，使得一个子数组中的所有元素都小于基准元素，另一个子数组中的所有元素都大于基准元素。
+ 递归地对这两个子数组进行排序。

快速排序是一种高效的排序算法，它的时间复杂度为O(n log n)，在平均情况下表现良好。然而，最坏情况下的时间复杂度为O(n^2)，这通常发生在数组已经部分有序的情况下。

```plaintext
quick_sort(array, low, high):
    if low < high,
        pivot_index = split(array, low, high)
        quick_sort(array, low, pivot_index - 1)
        quick_sort(array, pivot_index + 1, high)

split(array, low, high):
    pivot = array[high]
    i = low - 1
    
    for j in low..high - 1：
        if array[j] <= pivot，then
            i = i + 1
            swap array[i] 和 array[j]
    
    swap array[i + 1] 和 array[high]
    return i + 1
```
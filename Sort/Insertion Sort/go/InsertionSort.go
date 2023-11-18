package main

import "fmt"

func InsertionSort[T int](arr []T) {
	for index := 1; index < len(arr); index++ {
		currentValue := arr[index]
		sortedEndIndex := index - 1
		for sortedEndIndex >= 0 && arr[sortedEndIndex] > currentValue {
			arr[sortedEndIndex+1] = arr[sortedEndIndex]
			sortedEndIndex--
		}
		arr[sortedEndIndex+1] = currentValue
	}
}

func main() {
	var arr = []int{29, 10, 14, 37, 14}
	InsertionSort(arr)
	fmt.Println(arr)
}

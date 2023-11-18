package main

import "fmt"

func SelectionSort[T int](arr []T) {
	for i := range arr {
		minIndex := i
		for j := i + 1; j < len(arr); j++ {
			if arr[j] < arr[minIndex] {
				minIndex = j
			}
		}
		arr[i], arr[minIndex] = arr[minIndex], arr[i]
	}
}
func main() {
	var arr = []int{29, 10, 14, 37, 14}
	SelectionSort[int](arr)
	fmt.Printf("%v\n", arr)
}

package main

import "fmt"

func bubbleSort[T []int](arr T) T {
	for index := range arr {
		for i := 0; i < len(arr)-1-index; i++ {
			if arr[i] > arr[i+1] {
				arr[i], arr[i+1] = arr[i+1], arr[i]
			}
		}
	}
	return arr
}

func main() {
	arr := []int{29, 10, 14, 37, 14}
	start := time.Now()
	fmt.Println(bubbleSort(arr))
	t := time.Now()
}

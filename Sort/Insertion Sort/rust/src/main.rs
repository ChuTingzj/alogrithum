fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let current_value = arr[i];
        let mut sorted_end_plus_index = i;
        while sorted_end_plus_index > 0 && arr[sorted_end_plus_index - 1] > current_value {
            arr[sorted_end_plus_index] = arr[sorted_end_plus_index - 1];
            sorted_end_plus_index -= 1
        }
        arr[sorted_end_plus_index] = current_value
    }
}

fn main() {
    let mut arr = [29, 10, 14, 37, 14];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}

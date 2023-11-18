fn selection_sort(arr: &mut [i32]) {
    let length = arr.len();
    for i in 0..length {
        let mut min_index = i;
        for j in i + 1..length {
            if let true = arr.get(j) < arr.get(min_index) {
                min_index = j
            }
        }
        arr.swap(i, min_index)
    }
}

fn main() {
    let mut arr = [29, 10, 14, 37, 14];
    selection_sort(&mut arr);
    println!("{:?}", arr);
}

use std::time::Instant;

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr.get(j) > arr.get(j + 1) {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = [29, 10, 14, 37, 14];
    let flag = Instant::now();
    //0.000001
    bubble_sort(&mut arr);
    let elapsed_time = flag.elapsed();
    println!("{:?}", arr);
    println!("{}", elapsed_time.as_micros())
}

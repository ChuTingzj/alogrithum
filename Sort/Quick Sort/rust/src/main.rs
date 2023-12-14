fn quick_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return Vec::from(&arr[..]);
    }
    let pivot = arr[0];
    let left = arr.iter().filter(|&&x| x < pivot).collect::<Vec<&i32>>();
    let mut left = left.iter().map(|&&x| x).collect::<Vec<i32>>();
    let right = arr.iter().filter(|&&x| x > pivot).collect::<Vec<&i32>>();
    let mut right = right.iter().map(|&&x| x).collect::<Vec<i32>>();
    let pivot = Vec::from([pivot]);
    println!("{:?},{:?},{:?}", left, pivot, right);
    Vec::from([quick_sort(&mut left), pivot, quick_sort(&mut right)]).concat()
}

fn main() {
    let mut array = Vec::from([29, 10, 14, 37, 14]);
    let array = quick_sort(&mut array);
    println!("{:?}", array);
}

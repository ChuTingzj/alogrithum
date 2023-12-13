fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    while left.len() != 0 && right.len() != 0 {
        if left[0] <= right[0] {
            result.push(left[0]);
            left.remove(0);
        } else {
            result.push(right[0]);
            right.remove(0);
        }
    }
    if left.len() != 0 {
        result.append(left)
    }
    if right.len() != 0 {
        result.append(right)
    }
    result
}

fn merge_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 {
        return Vec::from(&arr[..]);
    }
    let mid_pointer = ((arr.len() / 2) as f32).ceil();
    let left = &arr[0..mid_pointer as usize];
    let mut left = Vec::from(left);
    let right = &arr[mid_pointer as usize..];
    let mut right = Vec::from(right);
    left = merge_sort(&mut left);
    right = merge_sort(&mut right);
    merge(&mut left, &mut right)
}

fn main() {
    let mut arr = Vec::from([29, 10, 14, 37, 14]);
    let arr_sorted = merge_sort(&mut arr);
    println!("{:?}", arr_sorted);
}

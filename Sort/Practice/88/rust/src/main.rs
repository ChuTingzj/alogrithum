mod solution;
fn main() {
    let mut nums1: Vec<i32> = Vec::from([1, 2, 3, 0, 0, 0]);
    let mut nums2: Vec<i32> = Vec::from([2, 5, 6]);
    solution::Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1)
}

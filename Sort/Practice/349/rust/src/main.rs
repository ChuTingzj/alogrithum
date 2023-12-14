mod solution;
fn main() {
    let mut nums1 = Vec::from([1, 2, 2, 1]);
    let mut nums2 = Vec::from([2, 2]);
    let intersection = solution::Solution::intersection_two_pointer(&mut nums1, &mut nums2);
    println!("{:?}", intersection);
}

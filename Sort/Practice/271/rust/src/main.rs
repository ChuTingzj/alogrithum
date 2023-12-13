mod solution;
fn main() {
    let nums = Vec::from([1, 5, -2, -4, 0]);
    let result = solution::Solution::contains_duplicate(nums);
    println!("{}", result)
}

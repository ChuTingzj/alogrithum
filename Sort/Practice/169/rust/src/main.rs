mod solution;
fn main() {
    let nums = Vec::from([3, 2, 3]);
    let num = solution::Solution::majority_element(nums);
    print!("{}", num)
}

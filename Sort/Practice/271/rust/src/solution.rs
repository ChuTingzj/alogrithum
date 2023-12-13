use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        for i in 0..nums.len() {
            if !set.insert(nums[i]) {
                return true;
            }
        }
        return false;
    }
}

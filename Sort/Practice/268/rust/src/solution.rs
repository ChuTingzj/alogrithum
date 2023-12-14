pub struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            match nums.iter().find(|&&x| x == i as i32) {
                Some(_) => {}
                None => return i as i32,
            }
        }
        return nums.len() as i32;
    }
}

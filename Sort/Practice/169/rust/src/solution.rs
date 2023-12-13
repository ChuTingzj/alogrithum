pub struct Solution;
impl Solution {
    fn find(nums: Vec<i32>, standard_line: f32, times: u32) -> i32 {
        let mut target: [i32; 2] = [nums[times as usize], 1];
        for i in (times + 1) as u32..nums.len() as u32 {
            if nums[i as usize] == target[0] {
                target[1] = target[1] + 1
            }
        }
        if target[1] > standard_line as i32 {
            target[0]
        } else {
            Solution::find(nums, standard_line, times + 1)
        }
    }
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            nums[0];
        }
        let standard_line = ((nums.len() / 2) as f32).floor();
        Solution::find(nums, standard_line, 0)
    }
}

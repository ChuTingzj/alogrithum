pub struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut merge_vec = [&mut nums1[0..m as usize], &mut nums2[0..n as usize]].concat();
        for o in 0..merge_vec.len() {
            for i in 0..merge_vec.len() - o - 1 {
                if merge_vec[i as usize] > merge_vec[(i + 1) as usize] {
                    merge_vec.swap(i as usize, (i + 1) as usize)
                }
            }
        }
        nums1.copy_from_slice(&merge_vec[..]);
    }
    pub fn merge_double_pointer(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut left_pointer = m - 1;
        let mut right_pointer = n - 1;
        let mut tail_pointer = m + n - 1;
        let mut temp = 0;
        while left_pointer >= 0 || right_pointer >= 0 {
            if left_pointer == -1 {
                temp = nums2[right_pointer as usize];
                right_pointer -= 1
            } else if right_pointer == -1 {
                temp = nums1[left_pointer as usize];
                left_pointer -= 1
            } else if nums1[left_pointer as usize] > nums2[right_pointer as usize] {
                temp = nums1[left_pointer as usize];
                left_pointer -= 1;
            } else {
                temp = nums2[right_pointer as usize];
                right_pointer -= 1;
            }
            nums1[tail_pointer as usize] = temp;
            tail_pointer -= 1
        }
    }
}

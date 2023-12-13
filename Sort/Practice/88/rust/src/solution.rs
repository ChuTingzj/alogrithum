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
        nums1.copy_from_slice(&merge_vec[..])
    }
}

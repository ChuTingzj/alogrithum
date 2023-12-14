pub struct Solution;
impl Solution {
    fn quick_sort(nums: &mut Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return Vec::from(&nums[..]);
        }
        let pivot = nums[0];
        let left = nums[1..]
            .iter()
            .filter(|&&x| x < pivot)
            .collect::<Vec<&i32>>();
        let mut left = left.iter().map(|&&x| x).collect::<Vec<i32>>();
        let right = nums[1..]
            .iter()
            .filter(|&&x| x >= pivot)
            .collect::<Vec<&i32>>();
        let mut right = right.iter().map(|&&x| x).collect::<Vec<i32>>();
        let pivot = Vec::from([pivot]);
        Vec::from(
            [
                Solution::quick_sort(&mut left),
                pivot,
                Solution::quick_sort(&mut right),
            ]
            .concat(),
        )
    }
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() == nums2.len() && nums1.len() == 1 {
            if nums1[0] == nums2[0] {
                return Vec::from(&nums1[..]);
            } else {
                return vec![] as Vec<i32>;
            }
        }
        let mut result: Vec<i32> = Vec::new();
        let (short, long) = {
            if nums1.len() > nums2.len() {
                (nums2, nums1)
            } else {
                (nums1, nums2)
            }
        };
        for i in 0..short.len() {
            match long.iter().find(|&&x| x == short[i]) {
                Some(v) => {
                    if !result.contains(v) {
                        result.push(*v)
                    }
                }
                None => {}
            }
        }
        result
    }
    pub fn intersection_two_pointer(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> Vec<i32> {
        if nums1.len() == nums2.len() && nums1.len() == 1 {
            if nums1[0] == nums2[0] {
                return Vec::from(&nums1[..]);
            } else {
                return vec![] as Vec<i32>;
            }
        }
        let sorted_nums1 = Solution::quick_sort(nums1);
        let sorted_nums2 = Solution::quick_sort(nums2);
        let mut nums1_pointer = 0;
        let mut nums2_pointer = 0;
        let mut result: Vec<i32> = Vec::new();
        while nums1_pointer < sorted_nums1.len() && nums2_pointer < sorted_nums2.len() {
            if sorted_nums1[nums1_pointer as usize] == sorted_nums2[nums2_pointer as usize]
                && !result.contains(&sorted_nums1[nums1_pointer as usize])
            {
                result.push(sorted_nums1[nums1_pointer as usize]);
                nums1_pointer += 1;
                nums2_pointer += 1;
            } else if sorted_nums1[nums1_pointer as usize] < sorted_nums2[nums2_pointer as usize] {
                nums1_pointer += 1;
            } else {
                nums2_pointer += 1;
            }
        }
        result
    }
}

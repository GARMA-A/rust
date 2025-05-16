use std::i32;

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut mx_sum = nums[0];
        let mut cur_sum: i32 = 0;
        for &num in nums.iter() {
            cur_sum = cur_sum.max(0) + num;
            mx_sum = mx_sum.max(cur_sum);
        }

        mx_sum
    }
}

#![allow(dead_code, unused_variables)]
pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut mx_reach = 0;
        for (i, &num) in nums.iter().enumerate() {
            mx_reach = mx_reach.max(num);
            if mx_reach == 0 && i < nums.len() - 1 {
                return false;
            }
            mx_reach -= 1;
        }

        true
    }
}

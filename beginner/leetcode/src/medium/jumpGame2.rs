#![allow(dead_code, unused_variables)]

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let (mut l, mut r) = (0 as i32, 0 as i32);
        while r < (nums.len() - 1) as i32 {
            let mut farthest = 0;
            for i in l..r + 1 {
                farthest = farthest.max(i + nums[i as usize]);
            }
            l = r + 1;
            r = farthest;
            jumps += 1;
        }

        jumps
    }
}

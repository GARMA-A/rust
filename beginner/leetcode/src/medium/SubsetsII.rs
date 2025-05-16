#![allow(dead_code, non_snake_case)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: HashSet<Vec<i32>> = HashSet::new();
        let mut nums = nums;
        nums.sort();
        Self::backtracking(&mut ans, &mut nums, &mut vec![], 0);
        ans.into_iter().collect()
    }
    pub fn backtracking(
        ans: &mut HashSet<Vec<i32>>,
        nums: &mut Vec<i32>,
        temp: &mut Vec<i32>,
        mut index: usize,
    ) {
        if index == nums.len() {
            ans.insert(temp.clone());
            return;
        };

        temp.push(nums[index]);
        Self::backtracking(ans, nums, temp, index + 1);
        temp.pop();
        while index + 1 < nums.len() && nums[index] == nums[index + 1] {
            index += 1;
        }
        Self::backtracking(ans, nums, temp, index + 1);
    }
}

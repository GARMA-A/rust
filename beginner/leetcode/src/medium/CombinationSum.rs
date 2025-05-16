#![allow(dead_code, non_snake_case)]
pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut candidates = candidates;
        candidates.sort();
        Self::backtracking(&candidates, target, &mut vec![], &mut ans, 0, 0);

        ans
    }

    pub fn backtracking(
        candidates: &Vec<i32>,
        target: i32,
        cur_combination: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        index: usize,
        curSum: i32,
    ) {
        if curSum == target {
            ans.push(cur_combination.clone());
            return;
        } else if curSum > target || index == candidates.len() {
            return;
        }

        cur_combination.push(candidates[index]);
        Self::backtracking(
            candidates,
            target,
            cur_combination,
            ans,
            index,
            curSum + candidates[index],
        );

        cur_combination.pop();
        Self::backtracking(candidates, target, cur_combination, ans, index + 1, curSum);
    }
}

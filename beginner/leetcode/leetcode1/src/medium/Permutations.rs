pub struct Solution;

impl Solution {
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut used = vec![false; nums.len()];
        let mut sub: Vec<i32> = Vec::new();
        
        Self::backtracking(&nums, &mut sub, &mut result, &mut used);
        
        result
    }

    fn backtracking(
        nums: &Vec<i32>,
        sub: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        used: &mut Vec<bool>,
    ) {
        if sub.len() == nums.len() {
            result.push(sub.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] { continue; }  

            used[i] = true;
            sub.push(nums[i]);
            Self::backtracking(nums, sub, result, used);
            used[i] = false;
            sub.pop();
        }
    }
}

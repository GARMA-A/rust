pub struct Solution;

impl Solution {

    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut sub: Vec<i32> = Vec::new();

        Self::backtracking(&mut nums, 0, &mut result, &mut sub);

        result
    }

    fn backtracking(
        nums: &mut Vec<i32>,
        index: usize,
        result: &mut Vec<Vec<i32>>,
        sub: &mut Vec<i32>,
    ) {
        if nums.len() == index {
            result.push(sub.clone());
            return;
        }
        sub.push(nums[index]);
        Self::backtracking(nums, index + 1, result, sub);
        sub.pop();
        Self::backtracking(nums, index + 1, result, sub);
    }
}
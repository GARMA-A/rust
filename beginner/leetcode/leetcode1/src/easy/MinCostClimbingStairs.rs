use std::cmp::min;
pub struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut memo: Vec<i32> = vec![-1; cost.len()];
        return min(
            Self.helper(&cost, 0, &mut memo),
            Self.helper(&cost, 1, &mut memo),
        );
    }

    pub fn helper(&self, cost: &Vec<i32>, index: usize, mut memo: &mut Vec<i32>) -> i32 {
        if index >= cost.len() {
            return 0;
        }
        if memo[index] != -1 {
            return memo[index];
        }
        memo[index] = cost[index]
            + min(
                self.helper(&cost, index + 1, &mut memo),
                self.helper(&cost, index + 2, &mut memo),
            );
        memo[index]
    }
}

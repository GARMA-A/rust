mod easy;
mod hard;
mod medium;

fn main() {
    let cost = vec![1, 2, 3];

    println!(
        "{}",
        easy::MinCostClimbingStairs::Solution::min_cost_climbing_stairs(cost)
    );
}

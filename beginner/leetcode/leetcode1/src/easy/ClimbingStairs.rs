pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut one: i32 = 1;
        let mut two: i32 = 1;

        for n in 2..n + 1 {
            let temp = one;
            one = one + two;
            two = temp;
        }
        return one;
    }
}

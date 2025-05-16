struct Solution;

impl Solution {
    pub fn hammingWeight(mut n: i32) -> i32 {
        let mut counter: i32 = 0;
        while n > 0 {
            counter += n & 1;
            n = n >> 1;
        }
        counter
    }
}

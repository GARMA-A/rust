#![allow(dead_code, non_snake_case)]
struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        return Self::solved(x, n as i64);
    }

    fn solved(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n < 0 {
            return 1.0 / Self::solved(x, -n);
        }
        let half = Self::solved(x, n / 2);
        if n % 2 == 0 {
            half * half
        } else {
            half * half * x
        }
    }
}

#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut sum = 0;
        while sum != 1 {
            if seen.contains(&sum) {
                return false;
            }
            seen.insert(sum);
            sum = 0;
            while n != 0 {
                sum += (n % 10) * (n % 10);
                n /= 10;
            }
            n = sum;
        }

        true
    }
}

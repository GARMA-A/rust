#![allow(unused_variables, non_snake_case)]

use std::collections::BinaryHeap;
struct Solution;

impl Solution {
    fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity((k + 1) as usize);
        for num in nums {
            heap.push(-num); // n*log(k)
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        -heap.pop().unwrap()
    }
}

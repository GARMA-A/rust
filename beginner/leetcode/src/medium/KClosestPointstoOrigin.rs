#![allow(non_snake_case, unused_variables)]

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for point in points.into_iter() {
            let dist = point[0] * point[0] + point[1] * point[1];
            heap.push((dist, point));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_, point)| point).collect()
    }
}

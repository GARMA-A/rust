#![allow(unused_variables, non_snake_case)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    k: i32,
    nums: Vec<i32>,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let cloned_nums = nums.clone();
        let mut struc = Self {
            k,
            nums,
            heap: BinaryHeap::new(),
        };
        for num in cloned_nums.iter() {
            struc.adding_to_heap(*num);
        }
        struc
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.adding_to_heap(val);
        match self.heap.peek().unwrap() {
            Reverse(val) => *val,
        }
    }
    fn adding_to_heap(&mut self, val: i32) {
        if self.heap.len() < self.k as usize {
            self.heap.push(Reverse(val));
            return;
        }
        if let Some(Reverse(top)) = self.heap.peek() {
            if val > *top {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        }
    }
}

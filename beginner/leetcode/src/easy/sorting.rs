#![allow(dead_code, non_snake_case)]

pub struct Sorting;

impl Sorting {
    pub fn new() -> Self {
        return Self;
    }

    pub fn MergeSort(&self, arr: &mut Vec<i32>) -> Vec<i32> {
        if arr.len() == 1 {
            return arr.clone();
        }
        // let mut vec = vec!['a', 'b', 'c'];
        // let vec2 = vec.split_off(1);
        // assert_eq!(vec, ['a']);
        // assert_eq!(vec2, ['b', 'c']);
        let mut right = arr.split_off(arr.len() / 2);
        let mut left = arr;
        let sorted_left = self.MergeSort(&mut left);
        let sorted_right = self.MergeSort(&mut right);
        self.merge(&sorted_left, &sorted_right)
    }

    fn merge(&self, left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
        let (mut first, mut second): (usize, usize) = (0, 0);
        let mut sorted_arr: Vec<i32> = Vec::with_capacity(left.len() + right.len());
        while first < left.len() && second < right.len() {
            if left[first] < right[second] {
                sorted_arr.push(left[first]);
                first += 1;
            } else {
                sorted_arr.push(right[second]);
                second += 1;
            }
        }
        while first < left.len() {
            sorted_arr.push(left[first]);
            first += 1;
        }
        while second < right.len() {
            sorted_arr.push(right[second]);
            second += 1;
        }
        sorted_arr
    }

    pub fn quick_sort(&self, arr: &mut Vec<i32>, l: usize, h: usize) {
        if arr.len() == 1 || l >= h {
            return;
        }
        let pivot = self.partition(l, h, arr);
        self.quick_sort(arr, l, pivot - 1);
        self.quick_sort(arr, pivot + 1, h);
    }

    fn partition(&self, mut left: usize, mut right: usize, arr: &mut Vec<i32>) -> usize {
        let pivot = left;
        left += 1;

        while left <= right {
            if arr[left] <= arr[pivot] {
                left += 1;
            } else if arr[right] > arr[pivot] {
                right -= 1;
            } else {
                arr.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        arr.swap(pivot, right);
        right
    }
}

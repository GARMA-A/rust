#[allow(dead_code, unused_variables, non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![]];
        let mut queue: VecDeque<i32> = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node.borrow().val);
            ans.push(vec![node.borrow().val]);
        }

        while let Some(node) = queue.remove(0) {}
        ans
    }
}

#[allow(dead_code, unused_variables, non_snake_case)]
#[derive(Debug, PartialEq, Eq, Clone)]
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
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        Self::dfs(&root, 0, &mut ans);

        ans
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, index: usize, ans: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            if index == ans.len() {
                ans.push(vec![]);
            }

            ans[index].push(node.borrow().val);
            Self::dfs(&node.borrow().left, index + 1, ans);
            Self::dfs(&node.borrow().right, index + 1, ans);
        }
    }
}

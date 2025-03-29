use ::std::cmp::max;
use std::{cell::RefCell, rc::Rc};

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

struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans: i32 = 0;
        Self::helper(&root, &mut ans);
        ans
    }

    pub fn helper(node: &Option<Rc<RefCell<TreeNode>>>, mut ans: &mut i32) -> i32 {
        if let Some(curNode) = node {
            let left = Self::helper(&curNode.borrow().left, ans);
            let right = Self::helper(&curNode.borrow().right, ans);
            *ans = max(*ans, left + right);
            return 1 + max(left, right);
        }
        0
    }
}

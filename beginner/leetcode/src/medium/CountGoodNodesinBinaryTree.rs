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
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut mx_val = i32::MIN;
        Self::dfs(&root, mx_val)
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut mx_val: i32) -> i32 {
        if let Some(node) = root {
            let mut count = 0;
            if node.borrow().val >= mx_val {
                count = 1;
            }
            mx_val = mx_val.max(node.borrow().val);
            count += Self::dfs(&node.borrow().left, mx_val);
            count += Self::dfs(&node.borrow().right, mx_val);
            return count;
        }
        0
    }
}

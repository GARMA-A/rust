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
use std::i32;
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::validate(&root, None, None);
    }

    pub fn validate(
        root: &Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;

            if let Some(min_val) = min {
                if min_val >= val {
                    return false;
                }
            }
            if let Some(max_val) = max {
                if max_val <= val {
                    return false;
                }
            }

            let left = &node.borrow().left;
            let right = &node.borrow().right;
            return Self::validate(left, min, Some(val)) && Self::validate(right, Some(val), max);
        }

        true
    }
}

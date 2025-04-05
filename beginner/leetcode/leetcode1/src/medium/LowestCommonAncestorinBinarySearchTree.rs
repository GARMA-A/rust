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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.as_ref().unwrap().borrow().val;
        let q = q.as_ref().unwrap().borrow().val;
        let mut root = root;
        while let Some(node) = root {
            let node_val = node.borrow().val;
            if p < node_val && q < node_val {
                root = node.borrow().left.clone();
            } else if p > node_val && q > node_val {
                root = node.borrow().right.clone();
            } else {
                return Some(node);
            }
        }
        None
    }
}

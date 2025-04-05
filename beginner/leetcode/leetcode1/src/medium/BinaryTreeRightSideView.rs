use ::std::cell::RefCell;
use ::std::rc::Rc;

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        Self::dfs(&root, &mut ans, 0);
        ans
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>, depth: i32) {
        if let Some(node) = root {
            if depth == ans.len() as i32 {
                ans.push(node.borrow().val);
            }
            Self::dfs(&node.borrow().right, ans, depth + 1);
            Self::dfs(&node.borrow().left, ans, depth + 1);
        }
    }
}

#![allow(dead_code, non_snake_case)]

use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub unsafe fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut temp: Option<Box<ListNode>> = head.clone();
        let mut nodes: VecDeque<i32> = VecDeque::new();
        while temp.is_some() {
            nodes.push_back(temp.clone().unwrap().val);
            temp = temp.unwrap().next;
        }
        let mut start = 1;
        let mut end = nodes.len() - 1;
        let mut ans = Some(Box::new(ListNode::new(nodes[0])));
        let mut currentNode = ans.as_mut();
        while start <= end {
            currentNode.as_mut().unwrap().next = Some(Box::new(ListNode::new(nodes[end])));
            currentNode = currentNode.unwrap().next.as_mut();
            if start < end {
                currentNode.as_mut().unwrap().next = Some(Box::new(ListNode::new(nodes[start])));
                currentNode = currentNode.unwrap().next.as_mut();
            }
            start += 1;
            end -= 1;
        }
    }
}

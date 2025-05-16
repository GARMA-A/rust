#![allow(dead_code)]
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

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut end = head.clone();
    let mut length = 1;
    while let Some(n) = end {
        if n.next == None {
            break;
        }
        end = n.next;
        length += 1;
    }
}


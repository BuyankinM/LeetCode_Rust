// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/

// Definition for singly-linked list.
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

use crate::Solution;

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = &head;
        let mut len = 0;
        while let Some(ref node) = ptr {
            len += 1;
            ptr = &node.next;
        }

        let mid = len / 2 - (1 - len % 2);
        while let Some(mut node) = head.take() {
            len -= 1;
            match len > mid {
                true => head = node.next.take(),
                false => return Some(node),
            }
        }
        unreachable!();
    }

    pub fn middle_node_jump2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = head.clone();
        while ptr.is_some() && ptr.as_ref()?.next.is_some() {
            ptr = ptr.unwrap().next.unwrap().next;
            head = head.unwrap().next;
        }
        head
    }

    pub fn middle_node_2cursors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_cur = &head;
        let mut slow_cur = &head;
        let mut idx = 0;
        while let Some(node) = fast_cur {
            if idx % 2 == 1 {
                slow_cur = &slow_cur.as_ref().unwrap().next;
            }
            fast_cur = &node.next;
            idx += 1;
        }
        slow_cur.clone()
    }
}

// 203. Remove Linked List Elements
// https://leetcode.com/problems/remove-linked-list-elements/

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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut prev_head = ListNode::new(0);
        let mut cur_node = &mut prev_head;

        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);
            if node.val != val {
                cur_node.next = Some(node);
                cur_node = cur_node.next.as_mut().unwrap();
            }
        }
        prev_head.next
    }

    // https://leetcode.com/problems/remove-linked-list-elements/discuss/746297/Rust-no-unwraps.
    pub fn remove_elements_opt(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;
        loop {
            match ptr {
                None => break,
                Some(node) if node.val == val => {
                    *ptr = node.next.take();
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }
        head
    }

    // https://leetcode.com/problems/remove-linked-list-elements/discuss/1573492/Rust-SImple-one-pass-solution
    pub fn remove_elements_some(
        mut head: Option<Box<ListNode>>,
        val: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut tail = &mut dummy;

        while let Some(mut node) = head {
            head = node.next.take();

            if node.val != val {
                *tail = Some(node);
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
        dummy
    }
}

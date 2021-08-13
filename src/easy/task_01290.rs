// 1290. Convert Binary Number in a Linked List to Integer
// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

use crate::Solution;

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

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut cur_node = head;
        let mut v = Vec::new();
        while let Some(node) = cur_node {
            v.push(node.val);
            cur_node = node.next;
        }
        v.iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (ind, val)| acc + *val * (1 >> ind))
    }

    // https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/discuss/454789/Rust-0ms-2.4MB
    pub fn get_decimal_value_only_bits(head: Option<Box<ListNode>>) -> i32 {
        let mut next = head;
        let mut val = 0;
        while let Some(node) = next {
            val = val << 1 | node.val;
            next = node.next;
        }
        val
    }

    // https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/discuss/918917/Rust-stream-solution
    pub fn get_decimal_value_successors(head: Option<Box<ListNode>>) -> i32 {
        std::iter::successors(head.as_ref(), |v| v.next.as_ref())
            .map(|v| v.val)
            .fold(0, |a, b| a * 2 + b)
    }
}

// 206. Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/

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
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn reverse_list(head: Node) -> Node {
        let (mut head, mut prev) = (head, None);
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn reverse_list_recursive(head: Node) -> Node {
        fn recurse(head: Node, prev: Node) -> Node {
            match head {
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = prev;
                    recurse(next, Some(node))
                }
                None => prev,
            }
        }
        recurse(head, None)
    }

    // https://leetcode.com/problems/reverse-linked-list/discuss/1814267/Rust-or-0ms-or-2.8-or
    pub fn reverse_list_mem(head: Node) -> Node {
        use std::mem::replace;
        let (mut prev, mut head) = (None, head);
        while let Some(mut node) = head {
            head = replace(&mut node.next, prev);
            prev = Some(node)
        }
        prev
    }
}

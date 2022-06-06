// 83. Remove Duplicates from Sorted List
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let mut head = head;
        let mut ptr = head.as_mut().unwrap();
        while let Some(ptr_next) = ptr.next.as_mut() {
            match ptr.val == ptr_next.val {
                true => ptr.next = ptr_next.next.take(),
                false => ptr = ptr.next.as_mut().unwrap(),
            }
        }
        head
    }

    // https://leetcode.com/problems/remove-duplicates-from-sorted-list/discuss/1557910/Rust-Simple-oror-Concise-oror-Iterative-oror-0ms-100
    pub fn delete_duplicates_opt(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr_opt = head.as_mut();

        while let Some(curr) = curr_opt {
            let mut next_opt = curr.next.take();
            while let Some(next) = next_opt.as_mut() {
                match next.val == curr.val {
                    true => next_opt = next.next.take(),
                    false => {
                        curr.next = next_opt;
                        break;
                    }
                }
            }
            curr_opt = curr.next.as_mut();
        }
        head
    }
}

// 21. Merge Two Sorted Lists
// https://leetcode.com/problems/merge-two-sorted-lists/

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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut zero_head = ListNode::new(0);
        let mut curr = &mut zero_head;

        loop {
            match (l1, l2) {
                (Some(mut v1), Some(mut v2)) => {
                    if v1.val < v2.val {
                        l1 = v1.next.take();
                        l2 = Some(v2);
                        curr.next = Some(v1);
                    } else {
                        l1 = Some(v1);
                        l2 = v2.next.take();
                        curr.next = Some(v2);
                    }
                }
                (Some(v1), None) => {
                    curr.next = Some(v1);
                    break;
                }
                (None, Some(v2)) => {
                    curr.next = Some(v2);
                    break;
                }
                (None, None) => break,
            }

            curr = curr.next.as_mut().unwrap();
        }

        zero_head.next
    }

    pub fn merge_two_lists_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(x), Some(y)) => Some(Box::new(if x.val <= y.val {
                ListNode {
                    val: x.val,
                    next: Solution::merge_two_lists(x.next, Some(y)),
                }
            } else {
                ListNode {
                    val: y.val,
                    next: Solution::merge_two_lists(Some(x), y.next),
                }
            })),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }

    pub fn merge_two_lists_swap(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::mem;

        let mut result = l1;
        let mut l2 = l2;
        let mut lsmall = &mut result;
        let lbig = &mut l2;
        
        while lbig.is_some() {
            if lsmall.is_none() || lsmall.as_ref()?.val > lbig.as_ref()?.val {
                mem::swap(lsmall, lbig);
            }
            if lsmall.is_some() {
                lsmall = &mut lsmall.as_mut()?.next;
            }
        }
        result
    }
}

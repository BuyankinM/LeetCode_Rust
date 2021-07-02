// 234. Palindrome Linked List
// https://leetcode.com/problems/palindrome-linked-list/

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
    pub fn is_palindrome_linked_list(mut head: Option<Box<ListNode>>) -> bool {
        let mut v: Vec<i32> = vec![];

        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }

        let l = v.len();
        (l == 1)
            || (l > 0)
                && (v[..l / 2]
                    .iter()
                    .zip(v[l / 2..].iter().rev())
                    .all(|(x, y)| x == y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut l1 = ListNode::new(1);
        let mut l2 = ListNode::new(2);
        let mut l3 = ListNode::new(2);
        let l4 = ListNode::new(1);

        l3.next = Some(Box::new(l4));
        l2.next = Some(Box::new(l3));
        l1.next = Some(Box::new(l2));

        assert_eq!(
            true,
            Solution::is_palindrome_linked_list(Some(Box::new(l1)))
        );
    }

    #[test]
    fn test_2() {
        let mut l1 = ListNode::new(1);
        l1.next = Some(Box::new(ListNode::new(2)));

        assert_eq!(
            false,
            Solution::is_palindrome_linked_list(Some(Box::new(l1)))
        );
    }
}

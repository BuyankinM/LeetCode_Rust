// 1302. Deepest Leaves Sum
// https://leetcode.com/problems/deepest-leaves-sum/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        let mut q = std::collections::VecDeque::new();
        q.push_front(root);

        while !q.is_empty() {
            sum = 0;
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop_back() {
                    let node = node.borrow();
                    sum += node.val;

                    if node.left.is_some() {
                        q.push_front(node.left.clone());
                    }
                    if node.right.is_some() {
                        q.push_front(node.right.clone());
                    }
                }
            }
        }
        sum
    }
}

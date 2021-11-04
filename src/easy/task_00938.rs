// 938. Range Sum of BST
// https://leetcode.com/problems/range-sum-of-bst/

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut res = 0;
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                if node.val >= low && node.val <= high {
                    res += node.val;
                }
                if node.val > low {
                    stack.push(node.left.clone());
                }
                if node.val < high {
                    stack.push(node.right.clone());
                }
            }
        }
        res
    }
}

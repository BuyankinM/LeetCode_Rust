// 404. Sum of Left Leaves
// https://leetcode.com/problems/sum-of-left-leaves/

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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut stack = vec![(root, false)];
        while let Some(node_flag) = stack.pop() {
            if let Some(node) = node_flag.0 {
                let node = node.borrow();
                if node_flag.1 && node.left.is_none() && node.right.is_none() {
                    res += node.val;
                }
                stack.push((node.left.clone(), true));
                stack.push((node.right.clone(), false));
            }
        }
        res
    }
}

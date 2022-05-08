// 112. Path Sum
// https://leetcode.com/problems/path-sum/

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack = vec![(root, 0)];
        while let Some((node, sum)) = stack.pop() {
            if let Some(rc_node) = node {
                let node = rc_node.borrow();
                let new_sum = sum + node.val;
                match new_sum == target_sum && node.left.is_none() && node.right.is_none() {
                    true => return true,
                    false => {
                        stack.push((node.left.clone(), new_sum));
                        stack.push((node.right.clone(), new_sum));
                    }
                }
            }
        }
        false
    }
}

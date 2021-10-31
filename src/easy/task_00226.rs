// 226. Invert Binary Tree
// https://leetcode.com/problems/invert-binary-tree/

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left_node = node.borrow().left.clone();
            let right_node = node.borrow().right.clone();
            node.borrow_mut().left = Self::invert_tree(right_node);
            node.borrow_mut().right = Self::invert_tree(left_node);
            return Some(node);
        }
        None
    }
}

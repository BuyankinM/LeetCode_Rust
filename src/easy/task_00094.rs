// 94. Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/

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
use std::rc::{self, Rc};
type OptionNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: OptionNode) -> Vec<i32> {
        fn recursive(node: OptionNode, v: &mut Vec<i32>) {
            if let Some(rc_node) = node {
                let node = rc_node.borrow();
                recursive(node.left.clone(), v);
                v.push(node.val);
                recursive(node.right.clone(), v);
            }
        }

        let mut v = vec![];
        recursive(root, &mut v);
        v
    }

    pub fn inorder_traversal_iterative(root: OptionNode) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || !stack.is_empty() {
            while let Some(rc_node) = node {
                stack.push(rc_node.clone());
                node = rc_node.borrow().left.clone();
            }
            if let Some(rc_node) = stack.pop() {
                res.push(rc_node.borrow().val);
                node = rc_node.borrow().right.clone();
            }
        }
        res
    }
}

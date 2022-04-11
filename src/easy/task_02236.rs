// 2236. Root Equals Sum of Children
// https://leetcode.com/problems/root-equals-sum-of-children/

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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_node = root.as_ref().unwrap().borrow();
        let left_node = root_node.left.as_ref().unwrap().borrow();
        let right_node = root_node.right.as_ref().unwrap().borrow();
        root_node.val == left_node.val + right_node.val
    }

    pub fn check_tree_safe(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(ref root_node) => {
                let root_node = root_node.borrow();
                let left_val = match root_node.left {
                    Some(ref left_node) => left_node.borrow().val,
                    None => panic!("Empty left node!"),
                };

                let right_val = match root_node.right {
                    Some(ref right_node) => right_node.borrow().val,
                    None => panic!("Empty right node!"),
                };

                root_node.val == left_val + right_val
            }
            None => panic!("Empty root!"),
        }
    }
}

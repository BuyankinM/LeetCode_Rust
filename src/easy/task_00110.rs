// 110. Balanced Binary Tree
// https://leetcode.com/problems/balanced-binary-tree/

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(rc_node) => {
                    let node = rc_node.borrow();
                    let ld = depth(node.left.clone());
                    let rd = depth(node.right.clone());
                    match ld > -1 && rd > -1 && (ld - rd).abs() <= 1 {
                        true => ld.max(rd) + 1,
                        false => -1,
                    }
                }
            }
        }
        depth(root) != -1
    }
}

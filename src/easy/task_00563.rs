// 563. Binary Tree Tilt
// https://leetcode.com/problems/binary-tree-tilt/

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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, sum_tilts: &mut i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let l = traverse(&node.left, sum_tilts);
                let r = traverse(&node.right, sum_tilts);
                *sum_tilts += (l - r).abs();
                l + r + node.val
            } else {
                0
            }
        }
        let mut sum_tilts = 0;
        traverse(&root, &mut sum_tilts);
        sum_tilts
    }
}

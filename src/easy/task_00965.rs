// 965. Univalued Binary Tree
// https://leetcode.com/problems/univalued-binary-tree/

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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut val = -1;
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(rc_node) = node {
                let ref_node = rc_node.borrow();
                match val {
                    -1 => val = ref_node.val,
                    _ if val != ref_node.val => return false,
                    _ => (),
                }
                stack.push(ref_node.left.clone());
                stack.push(ref_node.right.clone());
            }
        }
        true
    }
}

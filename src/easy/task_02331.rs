// 2331. Evaluate Boolean Binary Tree
// https://leetcode.com/problems/evaluate-boolean-binary-tree/

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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.as_ref().unwrap().borrow();
        match node.val {
            x @ (0 | 1) => x == 1,
            2 => {
                Solution::evaluate_tree(node.left.clone())
                    || Solution::evaluate_tree(node.right.clone())
            }
            _ => {
                Solution::evaluate_tree(node.left.clone())
                    && Solution::evaluate_tree(node.right.clone())
            }
        }
    }
}

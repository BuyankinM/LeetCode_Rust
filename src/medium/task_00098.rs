// 98. Validate Binary Search Tree
// https://leetcode.com/problems/validate-binary-search-tree/

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
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_valid_bst(root: OptNode) -> bool {
        fn dfs(node: OptNode, left_val: Option<i32>, right_val: Option<i32>) -> bool {
            if let Some(node) = node.as_ref() {
                let node = node.borrow();
                let cur_val = node.val;
                (left_val.is_none() || cur_val > left_val.unwrap())
                    && (right_val.is_none() || cur_val < right_val.unwrap())
                    && dfs(node.left.clone(), left_val, Some(cur_val))
                    && dfs(node.left.clone(), Some(cur_val), right_val)
            } else {
                true
            }
        }
        dfs(root, None, None)
    }
}

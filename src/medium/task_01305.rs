// 1305. All Elements in Two Binary Search Trees
// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/

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
    pub fn get_all_elements(root1: OptNode, root2: OptNode) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = vec![root1, root2];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                res.push(node.val);
                stack.push(node.left.clone());
                stack.push(node.right.clone());
            }
        }
        res.sort_unstable();
        res
    }
}

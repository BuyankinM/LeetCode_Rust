// 257. Binary Tree Paths
// https://leetcode.com/problems/binary-tree-paths/

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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = vec![];
        let mut stack = vec![(root, vec![])];

        while let Some((node, mut path)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                path.push(node.val.to_string());

                if node.left.is_none() && node.right.is_none() {
                    res.push(path.join("->"));
                } else {
                    stack.push((node.right.clone(), path.clone()));
                    stack.push((node.left.clone(), path));
                }
            }
        }
        res
    }
}

// 144. Binary Tree Preorder Traversal
// https://leetcode.com/problems/binary-tree-preorder-traversal/

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
    pub fn preorder_traversal(root: OptNode) -> Vec<i32> {
        fn preorder_traversal_recursive(root: OptNode, res: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                res.push(node.val);
                preorder_traversal_recursive(node.left.clone(), res);
                preorder_traversal_recursive(node.right.clone(), res);
            }
        }

        let mut res = vec![];
        preorder_traversal_recursive(root, &mut res);
        res
    }

    pub fn preorder_traversal_iterative(root: OptNode) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            if let Some(ref node_rc) = node {
                let cur_node = node_rc.borrow();
                res.push(cur_node.val);
                stack.push(cur_node.right.clone());
                stack.push(cur_node.left.clone());
            }
        }

        res
    }
}

// 145. Binary Tree Postorder Traversal
// https://leetcode.com/problems/binary-tree-postorder-traversal/

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
    pub fn postorder_traversal(root: OptNode) -> Vec<i32> {
        fn postorder_traversal_recursive(root: OptNode, res: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                postorder_traversal_recursive(node.left.clone(), res);
                postorder_traversal_recursive(node.right.clone(), res);
                res.push(node.val);
            }
        }

        let mut res = vec![];
        postorder_traversal_recursive(root, &mut res);
        res
    }

    pub fn postorder_traversal_iterative(root: OptNode) -> Vec<i32> {
        let mut s1 = vec![root];
        let mut s2 = vec![];

        while let Some(node) = s1.pop() {
            if let Some(ref node_rc) = node {
                let cur_node = node_rc.borrow();
                if cur_node.left.is_some() {
                    s1.push(cur_node.left.clone());
                }
                if cur_node.right.is_some() {
                    s1.push(cur_node.right.clone());
                }
            }
            s2.push(node);
        }

        let mut res = vec![];
        while let Some(node) = s2.pop() {
            if let Some(ref node_rc) = node {
                res.push(node_rc.borrow().val);
            }
        }
        res
    }
}

// 572. Subtree of Another Tree
// https://leetcode.com/problems/subtree-of-another-tree/

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
    pub fn is_subtree(root: OptNode, sub_root: OptNode) -> bool {
        fn recur(node: &OptNode, sub_node: &OptNode) -> bool {
            match node {
                Some(node_rc) => {
                    node == sub_node
                        || recur(&node_rc.borrow().left, sub_node)
                        || recur(&node_rc.borrow().right, sub_node)
                }
                None => false,
            }
        }
        recur(&root, &sub_root)
    }
}

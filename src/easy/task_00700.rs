// 700. Search in a Binary Search Tree
// https://leetcode.com/problems/search-in-a-binary-search-tree/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: OptNode,
    pub right: OptNode,
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
use std::cmp::Ordering;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn search_bst(root: OptNode, val: i32) -> OptNode {
        let mut node = root;
        while let Some(rc_node) = node.clone() {
            let cur_node = rc_node.borrow();
            match cur_node.val.cmp(&val) {
                Ordering::Equal => return node,
                Ordering::Less if cur_node.right.is_some() => node = cur_node.right.clone(),
                Ordering::Greater if cur_node.left.is_some() => node = cur_node.left.clone(),
                _ => break,
            }
        }
        None
    }
}

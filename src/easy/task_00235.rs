// 235. Lowest Common Ancestor of a Binary Search Tree
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

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
    pub fn lowest_common_ancestor(root: OptNode, p: OptNode, q: OptNode) -> OptNode {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        let (min, max) = match p_val > q_val {
            true => (q_val, p_val),
            false => (p_val, q_val),
        };

        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node_val = node.borrow().val;
                if node_val >= min && node_val <= max {
                    return Some(node);
                }
                match node_val > max {
                    true => stack.push(node.borrow().left.clone()),
                    false => stack.push(node.borrow().right.clone()),
                }
            }
        }
        unreachable!()
    }
}

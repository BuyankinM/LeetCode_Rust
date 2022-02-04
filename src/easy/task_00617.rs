// 617. Merge Two Binary Trees
// https://leetcode.com/problems/merge-two-binary-trees/

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
    pub fn merge_trees(root1: OptNode, root2: OptNode) -> OptNode {
        match (root1.is_none(), root2.is_none()) {
            (true, true) => return None,
            (true, false) => return root2,
            (false, true) => return root1,
            _ => (),
        }

        let mut stack = vec![(root1.clone(), root2)];
        while let Some(pair) = stack.pop() {
            let (node_1, node_2) = (pair.0.unwrap(), pair.1.unwrap());
            let mut n1 = node_1.borrow_mut();
            let n2 = node_2.borrow_mut();
            n1.val += n2.val;

            match (n1.left.is_some(), n2.left.is_some()) {
                (true, true) => stack.push((n1.left.clone(), n2.left.clone())),
                (false, true) => n1.left = n2.left.clone(),
                _ => (),
            }

            match (n1.right.is_some(), n2.right.is_some()) {
                (true, true) => stack.push((n1.right.clone(), n2.right.clone())),
                (false, true) => n1.right = n2.right.clone(),
                _ => (),
            }
        }
        root1
    }
}

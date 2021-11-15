// 897. Increasing Order Search Tree
// https://leetcode.com/problems/increasing-order-search-tree/

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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
            if let Some(cur_node) = node {
                let cur_node = cur_node.borrow();
                inorder(&cur_node.left, stack);
                stack.push(cur_node.val);
                inorder(&cur_node.right, stack);
            }
        }
        let mut stack = vec![];
        inorder(&root, &mut stack);
        stack.iter().rev().fold(None, |right, &val| {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                right,
                left: None,
            })))
        })
    }

    pub fn increasing_bst_iter(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        let dm = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = dm.clone();

        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                root = node.borrow_mut().right.take();
                node.borrow_mut().left = None;
                cur.borrow_mut().right = Some(node.clone());
                cur = node;
            }
        }

        let head = dm.borrow_mut().right.take();
        head
    }
}

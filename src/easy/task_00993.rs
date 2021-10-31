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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();

        if let Some(r) = root {
            q.push_back((r, None));
        }

        while !q.is_empty() {
            let (mut x_found, mut y_found) = (None, None);

            for _ in 0..q.len() {
                if let Some((node, parent)) = q.pop_front() {
                    let val = node.borrow().val;

                    if val == x {
                        x_found = parent;
                    }

                    if val == y {
                        y_found = parent;
                    }

                    if let Some(left_child) = node.borrow_mut().left.take() {
                        q.push_back((left_child, Some(val)));
                    }

                    if let Some(right_child) = node.borrow_mut().right.take() {
                        q.push_back((right_child, Some(val)));
                    }
                }
            }

            if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
                return x_parent != y_parent;
            }
        }
        false
    }
}

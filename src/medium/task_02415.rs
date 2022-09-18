// 2415. Reverse Odd Levels of Binary Tree
// https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/

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
    pub fn reverse_odd_levels(root: OptNode) -> OptNode {
        use std::collections::VecDeque;
        use std::mem::swap;

        let mut level_odd = false;
        let mut l = 1;

        let mut q_total = VecDeque::new();
        let mut q_level = VecDeque::new();
        q_total.push_back(root.clone());

        while !q_total.is_empty() {
            for _ in 0..l {
                if let Some(Some(node_rc)) = q_total.pop_front() {
                    if level_odd {
                        q_level.push_back(Some(node_rc.clone()));
                    }

                    let node = node_rc.borrow();
                    if node.left.is_some() {
                        q_total.push_back(node.left.clone());
                    }

                    if node.right.is_some() {
                        q_total.push_back(node.right.clone());
                    }
                }
            }

            if level_odd {
                for _ in 0..l / 2 {
                    let left = q_level.pop_front().unwrap().unwrap();
                    let right = q_level.pop_back().unwrap().unwrap();
                    swap(&mut left.borrow_mut().val, &mut right.borrow_mut().val);
                }
            }

            level_odd = !level_odd;
            l = q_total.len();
        }
        root
    }
}

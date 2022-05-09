// 111. Minimum Depth of Binary Tree
// https://leetcode.com/problems/minimum-depth-of-binary-tree/

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root.is_none() {
            true => 0,
            false => {
                let mut min_depth = i32::MAX;
                let mut stack = vec![(root, 1)];
                while let Some((node, depth)) = stack.pop() {
                    if let Some(rc_node) = node {
                        if depth >= min_depth {
                            continue;
                        }

                        let node = rc_node.borrow();
                        match node.left.is_none() && node.right.is_none() {
                            true => {
                                min_depth = min_depth.min(depth);
                                if min_depth == 2 {
                                    break;
                                }
                            }
                            false => {
                                stack.push((node.left.clone(), depth + 1));
                                stack.push((node.right.clone(), depth + 1));
                            }
                        }
                    }
                }
                min_depth
            }
        }
    }
}

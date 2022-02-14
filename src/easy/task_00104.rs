// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/

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
    pub fn max_depth_104(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut stack = vec![(root, 0)];
        while let Some((node, mut depth)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                depth += 1;
                max_depth = max_depth.max(depth);

                stack.push((node.left.clone(), depth));
                stack.push((node.right.clone(), depth));
            }
        }
        max_depth
    }

    //https://leetcode.com/problems/maximum-depth-of-binary-tree/discuss/616264/Rust-0ms
    pub fn max_depth_104_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(a) => {
                let l = Solution::max_depth_104_recur(a.borrow().left.clone());
                let r = Solution::max_depth_104_recur(a.borrow().right.clone());
                std::cmp::max(l, r) + 1
            }
            None => 0,
        }
    }
}

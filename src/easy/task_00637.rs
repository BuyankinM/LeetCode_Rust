// 637. Average of Levels in Binary Tree
// https://leetcode.com/problems/average-of-levels-in-binary-tree/

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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = Vec::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back(root);

        while !q.is_empty() {
            let (mut s, mut n): (f64, f64) = (0.0, 0.0);
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let node = node.borrow();
                    s += node.val as f64;
                    n += 1.0;
                    q.push_back(node.left.clone());
                    q.push_back(node.right.clone());
                }
            }
            if n > 0.0 {
                res.push(s / n);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let node_7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node_15 = Rc::new(RefCell::new(TreeNode::new(15)));
        let node_20 = Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(node_15),
            right: Some(node_7),
        }));
        let node_9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(node_9),
            right: Some(node_20),
        }));

        assert_eq!(
            vec![3.00000, 14.50000, 11.00000],
            Solution::average_of_levels(Some(root))
        );
    }
}

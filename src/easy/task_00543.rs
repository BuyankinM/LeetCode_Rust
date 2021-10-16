// 543. Diameter of Binary Tree
// https://leetcode.com/problems/diameter-of-binary-tree/

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 1;
        Self::recur(&root, &mut diameter);
        diameter - 1
    }

    fn recur(node: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        if let Some(cur_node) = node {
            let ref_node = cur_node.borrow();
            let left_path = Self::recur(&ref_node.left, diameter);
            let right_path = Self::recur(&ref_node.right, diameter);

            *diameter = (*diameter).max(left_path + right_path + 1);
            1 + left_path.max(right_path)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let node_3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node_4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node_5 = Rc::new(RefCell::new(TreeNode::new(5)));

        let node_2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node_4),
            right: Some(node_5),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node_2),
            right: Some(node_3),
        }));

        assert_eq!(3, Solution::diameter_of_binary_tree(Some(root)));
    }
}

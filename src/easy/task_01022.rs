// 1022. Sum of Root To Leaf Binary Numbers
// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/

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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recur(node: &Option<Rc<RefCell<TreeNode>>>, mut s: i32, res: &mut i32) {
            if let Some(cur_node) = node {
                let ref_node = cur_node.borrow();
                s = (s << 1) | ref_node.val;

                if ref_node.left.is_none() && ref_node.right.is_none() {
                    *res += s;
                    return;
                }

                recur(&ref_node.left, s, res);
                recur(&ref_node.right, s, res);
            }
        }

        let mut res = 0;
        recur(&root, 0, &mut res);
        res
    }

    pub fn sum_root_to_leaf_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recur(node: &Option<Rc<RefCell<TreeNode>>>, mut s: i32) -> i32 {
            if let Some(cur_node) = node {
                let ref_node = cur_node.borrow();
                s = (s << 1) | ref_node.val;

                match ref_node.left.is_none() && ref_node.right.is_none() {
                    true => s,
                    false => recur(&ref_node.left, s) + recur(&ref_node.right, s),
                }
            } else {
                0
            }
        }
        recur(&root, 0)
    }
}

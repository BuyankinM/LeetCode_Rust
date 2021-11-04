// 129. Sum Root to Leaf Numbers
// https://leetcode.com/problems/sum-root-to-leaf-numbers/

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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut stack = vec![(root, 0)];

        while let Some(node_sum) = stack.pop() {
            if let Some(node) = node_sum.0 {
                let node = node.borrow();
                let cur_sum = 10 * node_sum.1 + node.val;

                if node.left.is_none() && node.right.is_none() {
                    res += cur_sum;
                    continue;
                }

                stack.push((node.left.clone(), cur_sum));
                stack.push((node.right.clone(), cur_sum));
            }
        }
        res
    }
}

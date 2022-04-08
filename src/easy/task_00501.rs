// 501. Find Mode in Binary Search Tree
// https://leetcode.com/problems/find-mode-in-binary-search-tree/

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
    pub fn find_mode(root: OptNode) -> Vec<i32> {
        use std::collections::HashMap;

        let mut mode = 0;
        let mut res = Vec::new();

        let mut stack = vec![root];
        let mut counter_map = HashMap::new();

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                let val = node.val;
                let num = counter_map.entry(val).or_insert(0);
                *num += 1;

                if *num >= mode {
                    if *num > mode {
                        res.clear();
                        mode = *num;
                    }
                    res.push(val);
                }

                stack.push(node.left.clone());
                stack.push(node.right.clone());
            }
        }
        res
    }
}

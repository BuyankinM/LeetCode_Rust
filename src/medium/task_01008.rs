// 1008. Construct Binary Search Tree from Preorder Traversal
// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/

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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recur(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(val) = v.first() {
                let i = (1..v.len()).find(|&i| v[i] > v[0]).unwrap_or(v.len());

                let node = Rc::new(RefCell::new(TreeNode {
                    val: *val,
                    left: recur(&v[1..i]),
                    right: recur(&v[i..]),
                }));

                Some(node)
            } else {
                None
            }
        }

        recur(&preorder)
    }
}

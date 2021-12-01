// 783. Minimum Distance Between BST Nodes
// https://leetcode.com/problems/minimum-distance-between-bst-nodes/

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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            if let Some(node) = node {
                traverse(&node.borrow().left, v);
                v.push(node.borrow().val);
                traverse(&node.borrow().right, v);
            }
        }

        let mut v = Vec::new();
        traverse(&root, &mut v);

        let mut min_diff = i32::MAX;
        for pair in v.windows(2) {
            match pair[1] - pair[0] {
                1 => return 1,
                diff if diff < min_diff => min_diff = diff,
                _ => (),
            }
        }
        min_diff
    }

    // https://leetcode.com/problems/minimum-distance-between-bst-nodes/discuss/482190/Rust
    pub fn min_diff_in_bst_best(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn h(r: &Option<Rc<RefCell<TreeNode>>>, pre: &mut i32, min: &mut i32) {
            if let Some(r) = r {
                let r = r.borrow();
                h(&r.left, pre, min);
                *min = (r.val.saturating_sub(*pre)).min(*min);
                *pre = r.val; // !important;
                h(&r.right, pre, min);
            }
        }
        let mut min = i32::MAX;
        let mut pre = i32::MIN;
        h(&root, &mut pre, &mut min);
        min
    }
}

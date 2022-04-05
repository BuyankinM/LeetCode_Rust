// 530. Minimum Absolute Difference in BST
// https://leetcode.com/problems/minimum-absolute-difference-in-bst/

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
use std::i32::{MAX, MIN};
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn get_minimum_difference(root: OptNode) -> i32 {
        fn in_order_traverse(node: &OptNode, v: &mut Vec<i32>) {
            if let Some(node) = node {
                let node = node.borrow();
                in_order_traverse(&node.left, v);
                v.push(node.val);
                in_order_traverse(&node.right, v);
            }
        }

        let mut min_diff = MAX;
        let mut v = Vec::new();
        in_order_traverse(&root, &mut v);

        v.iter()
            .zip(v[1..].iter())
            .for_each(|(&x, &y)| min_diff = min_diff.min(y - x));

        min_diff
    }

    // https://leetcode.com/problems/minimum-absolute-difference-in-bst/discuss/846717/Rust-cheapest-and-best
    pub fn get_minimum_difference_opt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn go(root: &Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
            match root {
                Some(node) => {
                    let b = node.borrow();
                    let (mut last, mut res) = go(&b.left, last, res);
                    res = res.min(b.val.saturating_sub(last));
                    last = b.val;
                    go(&b.right, last, res)
                }
                _ => (last, res),
            }
        }
        go(&root, MIN, MAX).1
    }
}

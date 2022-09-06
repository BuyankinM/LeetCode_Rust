// 814. Binary Tree Pruning
// https://leetcode.com/problems/binary-tree-pruning/

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
    pub fn prune_tree(root: OptNode) -> OptNode {
        fn dfs(node: &OptNode) -> i32 {
            match node {
                Some(rc_node) => {
                    let mut node = rc_node.borrow_mut();
                    let left_val = dfs(&node.left);
                    let right_val = dfs(&node.right);
                    if left_val == 0 {
                        node.left = None;
                    }
                    if right_val == 0 {
                        node.right = None;
                    }
                    node.val.max(left_val).max(right_val)
                }
                _ => 0,
            }
        }

        match dfs(&root) {
            1 => root,
            _ => None,
        }
    }

    // https://leetcode.com/problems/binary-tree-pruning/discuss/2538742/Rust-or-Recursive-DFS-or-With-Comments
    pub fn prune_tree_short(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node_rc) => {
                let mut node_ref = node_rc.borrow_mut();
                match (
                    node_ref.val,
                    Self::prune_tree(node_ref.left.take()),
                    Self::prune_tree(node_ref.right.take()),
                ) {
                    (0, None, None) => None,
                    (_, left, right) => {
                        node_ref.left = left;
                        node_ref.right = right;
                        drop(node_ref);
                        Some(node_rc)
                    }
                }
            }
        }
    }

    // https://leetcode.com/problems/binary-tree-pruning/discuss/2417102/Rust-cheapest-and-best
    pub fn prune_tree_func(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let val = node.borrow().val;
            let right = Self::prune_tree(node.borrow_mut().right.take());
            let left = Self::prune_tree(node.borrow_mut().left.take());

            (!(val == 0 && right.is_none() && left.is_none())).then(|| {
                node.borrow_mut().right = right;
                node.borrow_mut().left = left;
                node
            })
        })
    }
}

// 222. Count Complete Tree Nodes
// https://leetcode.com/problems/count-complete-tree-nodes/

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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) {
            if let Some(node) = root {
                let ref_node = node.borrow();
                dfs(&ref_node.left, res);
                dfs(&ref_node.right, res);
                *res += 1;
            }
        }

        let mut res = 0;
        dfs(&root, &mut res);
        res
    }

    // https://leetcode.com/problems/count-complete-tree-nodes/discuss/514004/Rust-recursive-solution
    pub fn count_nodes_optimal(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let ld = {
                let mut h = 1;
                let mut node = r.borrow().left.clone();
                while let Some(n) = node {
                    h += 1;
                    node = n.borrow().left.clone();
                }
                h
            };
            let rd = {
                let mut h = 1;
                let mut node = r.borrow().right.clone();
                while let Some(n) = node {
                    h += 1;
                    node = n.borrow().right.clone();
                }
                h
            };
            if ld == rd {
                2i32.pow(ld) - 1
            } else {
                1 + Solution::count_nodes_optimal(r.borrow().left.clone())
                    + Solution::count_nodes_optimal(r.borrow().right.clone())
            }
        } else {
            0
        }
    }
}

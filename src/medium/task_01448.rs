// 1448. Count Good Nodes in Binary Tree
// https://leetcode.com/problems/count-good-nodes-in-binary-tree/

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut stack = vec![(root, i32::MIN)];
        while let Some((node, mut max_val)) = stack.pop() {
            if let Some(rc_node) = node {
                let node = rc_node.borrow();
                if node.val >= max_val {
                    res += 1;
                    max_val = max_val.max(node.val);
                }
                stack.push((node.left.clone(), max_val));
                stack.push((node.right.clone(), max_val));
            }
        }
        res
    }

    // https://leetcode.com/problems/count-good-nodes-in-binary-tree/discuss/1408601/Rust-DFS-solution
    pub fn good_nodes_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: Option<i32>) -> i32 {
            if let Some(n) = node {
                let val = n.borrow().val;
                let m = Some(max.map_or(val, |m| m.max(val)));
                (m == Some(val)) as i32 + dfs(&n.borrow().left, m) + dfs(&n.borrow().right, m)
            } else {
                0
            }
        }
        dfs(&root, None)
    }
}

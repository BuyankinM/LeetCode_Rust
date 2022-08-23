// 2385. Amount of Time for Binary Tree to Be Infected
// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/

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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let graph = Solution::collect_graph(root);

        let mut max_lev = 0;
        let mut stack = vec![(start, 0)];
        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some((node, lev)) = stack.pop() {
            if let Some(nodes) = graph.get(&node) {
                for &next_node in nodes.iter().filter(|&&n| n > -1) {
                    if visited.insert(next_node) {
                        let next_lev = lev + 1;
                        stack.push((next_node, next_lev));
                        if next_lev > max_lev {
                            max_lev = next_lev;
                        }
                    }
                }
            }
        }
        max_lev
    }

    pub fn collect_graph(root: Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut stack = vec![(root, -1)];

        while let Some((node, prev_val)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                let val = node.val;

                map.entry(val)
                    .and_modify(|v| v.push(prev_val))
                    .or_insert_with(|| vec![prev_val]);

                map.entry(prev_val)
                    .and_modify(|v| v.push(val))
                    .or_insert_with(|| vec![val]);

                stack.push((node.left.clone(), val));
                stack.push((node.right.clone(), val));
            }
        }
        map
    }
}

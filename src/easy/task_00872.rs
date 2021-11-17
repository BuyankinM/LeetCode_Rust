// 872. Leaf-Similar Trees
// https://leetcode.com/problems/leaf-similar-trees/

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
    pub fn leaf_similar(root1: OptNode, root2: OptNode) -> bool {
        fn traverse(node: &OptNode, v: &mut Vec<i32>) {
            if let Some(cur_node) = node {
                let cur_node = cur_node.borrow();
                if cur_node.left.is_none() && cur_node.right.is_none() {
                    v.push(cur_node.val);
                    return;
                }
                traverse(&cur_node.left, v);
                traverse(&cur_node.right, v);
            }
        }
        let (mut seq_1, mut seq_2) = (Vec::new(), Vec::new());
        traverse(&root1, &mut seq_1);
        traverse(&root2, &mut seq_2);
        seq_1 == seq_2
    }

    // https://leetcode.com/problems/leaf-similar-trees/discuss/1002755/Rust%3A-recursive-solution
    pub fn leaf_similar_short(root1: OptNode, root2: OptNode) -> bool {
        fn collect_leaves(n: OptNode) -> Vec<i32> {
            match n {
                None => vec![],
                Some(n) => {
                    let n = n.borrow();
                    if n.left.is_none() && n.right.is_none() {
                        return vec![n.val];
                    }
                    let mut list = collect_leaves(n.left.clone());
                    list.extend(collect_leaves(n.right.clone()));
                    list
                }
            }
        }
        collect_leaves(root1) == collect_leaves(root2)
    }
}

// 230. Kth Smallest Element in a BST
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/

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
    pub fn kth_smallest(root: OptNode, mut k: i32) -> i32 {
        fn inorder(root: OptNode, k: &mut i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let left = inorder(node.left.clone(), k);
                if left != -1 {
                    return left;
                }

                *k -= 1;
                if *k == 0 {
                    return node.val;
                }

                let right = inorder(node.right.clone(), k);
                if right != -1 {
                    return right;
                }
            }
            -1
        }
        inorder(root, &mut k)
    }

    // https://leetcode.com/problems/kth-smallest-element-in-a-bst/discuss/641848/Rust-O(h%2Bk)-minimalist-iterative-solution-with-explanations.-(0ms-faster-than-100.)
    pub fn kth_smallest_iterative(mut node_opt: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut nodes = Vec::new();
        while node_opt.is_some() || !nodes.is_empty() {
            // Go to the smallest (i.e. left-most) node:
            while let Some(node) = node_opt {
                nodes.push(node.clone());
                node_opt = node.borrow().left.clone();
            }
            if let Some(node) = nodes.pop() {
                // nodes contains Rc<RefCell<TreeNode>>s but pop returns Option<T>.
                // Check the current smallest node:
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                // Check the right sub-tree:
                node_opt = node.borrow().right.clone();
            }
        }
        unreachable!();
    }
}

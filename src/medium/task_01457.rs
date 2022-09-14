// 1457. Pseudo-Palindromic Paths in a Binary Tree
// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/

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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut stack = vec![(root, [0; 10])];

        while let Some((node, mut counter)) = stack.pop() {
            if let Some(node_rc) = node {
                let mut node = node_rc.borrow_mut();
                counter[node.val as usize] += 1;

                if node.left.is_none() && node.right.is_none() {
                    // leaf node
                    res += (counter.iter().filter(|&&x| x % 2 == 1).count() <= 1) as i32;
                } else {
                    stack.push((node.left.take(), counter));
                    stack.push((node.right.take(), counter));
                }
            }
        }
        res
    }

    // https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/discuss/2574162/Rust-or-Iterative-DFS-or-With-Comments
    pub fn pseudo_palindromic_paths_bitmasking(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root.unwrap(), 0u16)];
        let mut rez = 0;

        while let Some((node_rc, mut odds)) = stack.pop() {
            let node_ref = node_rc.borrow();
            odds ^= 1 << node_ref.val;
            match (node_ref.left.clone(), node_ref.right.clone()) {
                (None, None) => {
                    if odds.count_ones() < 2 {
                        rez += 1
                    }
                }
                (None, Some(right)) => stack.push((right, odds)),
                (Some(left), None) => stack.push((left, odds)),
                (Some(left), Some(right)) => {
                    stack.push((left, odds));
                    stack.push((right, odds));
                }
            }
        }

        rez
    }

    // https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/discuss/2574376/RUST-Itertaive-and-recursive-solutions
    pub fn pseudo_palindromic_paths_bitmasking_trick(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push((node, 0usize));
        }

        let mut count = 0;
        while let Some((node, freq)) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            // We can track the number of odd frequencies by using bitmasks.
            // I.e. a number I is odd, if thr Ith bit is one. Thus, we can easily
            // toggle between odd/even by just XORing this bit
            let freq = freq ^ (1 << node_ref.val);

            // If it's a leaf node, then update the answer
            if node_ref.left.is_none() && node_ref.right.is_none() {
                // A path is pseudo palindromic, if it has at most 1
                // odd frequency, i.e. this bitmask represents a power of two
                count += ((freq & (freq - 1)) == 0) as i32;
                continue;
            }

            // If it's not a leaf node, then push its children to the stack
            if let Some(right) = node_ref.right.take() {
                stack.push((right, freq));
            }
            if let Some(left) = node_ref.left.take() {
                stack.push((left, freq));
            }
        }

        count
    }
}

// 101. Symmetric Tree
// https://leetcode.com/problems/symmetric-tree/

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
type OptionNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: OptionNode) -> bool {
        use std::collections::VecDeque;

        let mut q = VecDeque::new();
        q.push_front(root);

        loop {
            let mut pr = false;
            let l = q.len();
            let mut v = vec![];

            for _ in 0..l {
                let opt_node = q.pop_back().unwrap();
                match opt_node {
                    None => v.push(999),
                    Some(rc_node) => {
                        let node = rc_node.borrow();

                        q.push_front(node.left.clone());
                        q.push_front(node.right.clone());

                        v.push(node.val);
                        pr = pr || node.left.is_some() || node.right.is_some();
                    }
                }
            }

            if !v.iter().take(l / 2).eq(v.iter().rev().take(l / 2)) {
                return false;
            }

            if !pr {
                break true;
            }
        }
    }

    // https://leetcode.com/problems/symmetric-tree/discuss/1229761/Rust%3A-recursive-solution
    pub fn is_symmetric_recur(root: OptionNode) -> bool {
        fn compare(l: OptionNode, r: OptionNode) -> bool {
            match (l, r) {
                (None, None) => true,
                (Some(left), Some(right)) => {
                    let l = left.borrow();
                    let r = right.borrow();
                    l.val == r.val
                        && compare(l.left.clone(), r.right.clone())
                        && compare(l.right.clone(), r.left.clone())
                }
                _ => false,
            }
        }
        match root {
            Some(r) => compare(r.borrow().left.clone(), r.borrow().right.clone()),
            None => true,
        }
    }
}

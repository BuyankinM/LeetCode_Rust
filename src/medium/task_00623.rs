// 623. Add One Row to Tree
// https://leetcode.com/problems/add-one-row-to-tree/

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
    pub fn add_one_row(root: OptNode, val: i32, depth: i32) -> OptNode {
        let new_node = |left: OptNode, right: OptNode, val: i32| -> OptNode {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        };

        if depth == 1 {
            return new_node(root, None, val);
        }

        let mut stack = vec![(root.clone(), 1)];

        while let Some((Some(rc_node), d)) = stack.pop() {
            let mut node = rc_node.borrow_mut();
            if d == depth - 1 {
                node.left = new_node(node.left.take(), None, val);
                node.right = new_node(None, node.right.take(), val);
            } else {
                if node.left.is_some() {
                    stack.push((node.left.clone(), d + 1));
                }
                if node.right.is_some() {
                    stack.push((node.right.clone(), d + 1));
                }
            }
        }
        root
    }

    // https://leetcode.com/problems/add-one-row-to-tree/discuss/2663188/Rust-clean-solution
    pub fn add_one_row_match(root: OptNode, new_val: i32, depth: i32) -> OptNode {
        Some(Rc::new(RefCell::new(match depth {
            1 => TreeNode {
                val: new_val,
                left: root,
                right: None,
            },
            2 => {
                let TreeNode { val, left, right } = Rc::try_unwrap(root?).unwrap().into_inner();
                TreeNode {
                    val,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: new_val,
                        left,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: new_val,
                        left: None,
                        right,
                    }))),
                }
            }
            depth => {
                let TreeNode { val, left, right } = Rc::try_unwrap(root?).unwrap().into_inner();
                TreeNode {
                    val,
                    left: Self::add_one_row(left, new_val, depth - 1),
                    right: Self::add_one_row(right, new_val, depth - 1),
                }
            }
        })))
    }

    // https://leetcode.com/problems/add-one-row-to-tree/discuss/2663461/Rust-or-BFS-or-With-Comments
    pub fn add_one_row_bfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;

        let dummy = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: root,
            right: None,
        }));
        let mut q = std::iter::once(dummy.clone()).collect::<VecDeque<_>>();
        for _ in 1..depth {
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node_rc = node.borrow();
                q.extend(
                    std::iter::once(node_rc.left.clone())
                        .chain(std::iter::once(node_rc.right.clone()))
                        .flatten(),
                );
            }
        }
        for _ in 0..q.len() {
            let node = q.pop_front().unwrap();
            let mut node_rc = node.borrow_mut();
            node_rc.left = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: node_rc.left.take(),
                right: None,
            })));
            node_rc.right = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: node_rc.right.take(),
            })));
        }
        let mut dummy_ref = dummy.borrow_mut();
        dummy_ref.left.take()
    }
}

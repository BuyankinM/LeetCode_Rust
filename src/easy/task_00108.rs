// 108. Convert Sorted Array to Binary Search Tree
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> OptionNode {
        fn recursive(nums: &[i32], start: usize, end: usize) -> OptionNode {
            match start == end {
                true => None,
                false => {
                    let mid = (start + end) / 2;
                    let mut node = TreeNode::new(nums[mid]);
                    node.left = recursive(nums, start, mid);
                    node.right = recursive(nums, mid + 1, end);
                    Some(Rc::new(RefCell::new(node)))
                }
            }
        }
        recursive(&nums, 0, nums.len())
    }
}

// 437. Path Sum III
// https://leetcode.com/problems/path-sum-iii/

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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
            let mut res = 0;
            if let Some(r) = root {
                let val = r.borrow().val;
                if val == target {
                    res += 1;
                }
                res += traverse(r.borrow().left.clone(), target - val);
                res += traverse(r.borrow().right.clone(), target - val);
            }
            res
        }

        let mut res = 0;
        res += traverse(root.clone(), sum);

        if let Some(node) = root {
            res += Self::path_sum(node.borrow().left.clone(), sum);
            res += Self::path_sum(node.borrow().right.clone(), sum);
        }

        res
    }

    pub fn path_sum_hashmap(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn helper(
            root: Option<&Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            table: &mut HashMap<i32, i32>,
            cur_sum: i32,
        ) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let mut res = 0;
                    let cur = cur_sum + node.borrow().val;
                    let prev = table.get(&(cur - target_sum));

                    if let Some(p_node) = prev {
                        res += *p_node;
                    }

                    *table.entry(cur).or_insert(0) += 1;

                    res += helper(node.borrow().left.as_ref(), target_sum, table, cur);
                    res += helper(node.borrow().right.as_ref(), target_sum, table, cur);

                    *table.get_mut(&cur).unwrap() -= 1;
                    res
                }
            }
        }

        let mut table: HashMap<i32, i32> = HashMap::new();
        table.entry(0).or_insert(1);
        helper(root.as_ref(), target_sum, &mut table, 0)
    }
}

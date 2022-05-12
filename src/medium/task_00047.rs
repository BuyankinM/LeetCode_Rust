// 47. Permutations II
// https://leetcode.com/problems/permutations-ii/

use crate::Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        fn permute_recursive(
            nums: &Vec<i32>,
            set_comb: &mut HashSet<Vec<i32>>,
            set_idx: HashSet<usize>,
            v: Vec<i32>,
        ) {
            for i in (0..nums.len()).filter(|i| !set_idx.contains(i)) {
                let mut next_v = v.clone();
                next_v.push(nums[i]);

                let mut next_set_idx = set_idx.clone();
                next_set_idx.insert(i);

                match next_set_idx.len() == nums.len() {
                    true => {
                        set_comb.insert(next_v);
                    }
                    false => permute_recursive(nums, set_comb, next_set_idx, next_v),
                }
            }
        }

        let mut set_comb = HashSet::new();
        permute_recursive(&nums, &mut set_comb, HashSet::new(), Vec::new());
        set_comb.into_iter().collect()
    }

    // aka_npou 
    pub fn permute_unique_opt(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        fn permute_recursive(nums: &mut Vec<i32>, start: usize, result: &mut HashSet<Vec<i32>>) {
            match start == nums.len() {
                true => {
                    result.insert(nums.clone());
                }
                false => {
                    for i in start..nums.len() {
                        nums.swap(i, start);
                        permute_recursive(nums, start + 1, result);
                        nums.swap(i, start);
                    }
                }
            }
        }

        let mut set_comb = HashSet::new();
        let mut nums = nums;
        permute_recursive(&mut nums, 0, &mut set_comb);
        set_comb.into_iter().collect()
    }
}

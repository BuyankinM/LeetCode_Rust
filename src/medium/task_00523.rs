// 523. Continuous Subarray Sum
// https://leetcode.com/problems/continuous-subarray-sum/

use crate::Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        hm.insert(0, 0);

        let mut s = 0;
        for (i, &x) in nums.iter().enumerate() {
            s += x;
            match hm.entry(s % k) {
                Entry::Vacant(e) => {
                    e.insert(i + 1);
                }
                Entry::Occupied(e) => {
                    if *(e.get()) < i {
                        return true;
                    }
                }
            }
        }
        false
    }

    // https://leetcode.com/problems/continuous-subarray-sum/discuss/2745274/Rust-or-Hash-Set-or-With-Comments
    pub fn check_subarray_sum_func(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;

        nums.into_iter()
            .scan((HashSet::<i32>::new(), 0), |(set, prefix_sum), num| {
                let new_prefix_sum = (*prefix_sum + num) % k;
                set.contains(&new_prefix_sum).then_some(true).or_else(|| {
                    set.insert(*prefix_sum);
                    *prefix_sum = new_prefix_sum;
                    Some(false)
                })
            })
            .any(|found| found)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn test_2() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }
}

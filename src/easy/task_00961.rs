// 961. N-Repeated Element in Size 2N Array
// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/

use crate::Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(nums.len() / 2 + 1);
        for &val in &nums {
            if !set.insert(val) {
                return val;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::repeated_n_times(vec![1, 2, 3, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]));
    }
}

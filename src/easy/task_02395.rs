// 2395. Find Subarrays With Equal Sum
// https://leetcode.com/problems/find-subarrays-with-equal-sum/

use crate::Solution;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());
        nums.iter()
            .zip(nums[1..].iter())
            .any(|(x, y)| !set.insert(*x + *y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::find_subarrays(vec![4, 2, 4]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::find_subarrays(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::find_subarrays(vec![0, 0, 0]));
    }
}

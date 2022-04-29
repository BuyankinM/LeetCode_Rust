// 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/

use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());
        nums.iter().any(|&num| !set.insert(num))
    }

    pub fn contains_duplicate_sort(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        nums.iter().zip(nums.iter().skip(1)).any(|(&a, &b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}

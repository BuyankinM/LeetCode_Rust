// 128. Longest Consecutive Sequence
// https://leetcode.com/problems/longest-consecutive-sequence/

use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = nums.iter().cloned().collect::<HashSet<_>>();
        let mut res = 0;
        for &num in &set {
            if !set.contains(&(num - 1)) {
                let count = (num..).take_while(|x| set.contains(x)).count();
                res = res.max(count);
            }
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }
}

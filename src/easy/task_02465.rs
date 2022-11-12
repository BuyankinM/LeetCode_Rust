// 2465. Number of Distinct Averages
// https://leetcode.com/problems/number-of-distinct-averages/

use crate::Solution;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        nums.sort_unstable();

        let mut it = nums.iter();

        (0..nums.len() / 2)
            .map(|_| *it.next().unwrap() + *it.next_back().unwrap())
            .collect::<HashSet<_>>()
            .len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::distinct_averages(vec![1, 2, 2, 1, 1, 0]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::distinct_averages(vec![1, 100]), 1);
    }
}

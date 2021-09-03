// 1984. Minimum Difference Between Highest and Lowest of K Scores
// https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/

use crate::Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.windows(k as usize)
            .map(|pair| pair[(k - 1) as usize] - pair[0])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::minimum_difference(vec![90], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::minimum_difference(vec![9, 4, 1, 7], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(8, Solution::minimum_difference(vec![9, 4, 1, 7], 4));
    }
}

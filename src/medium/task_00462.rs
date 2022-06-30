// 462. Minimum Moves to Equal Array Elements II
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/

use crate::Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let mid = nums.len() / 2;
        nums.select_nth_unstable(mid);

        let median = nums[mid];
        nums.iter().fold(0, |acc, &x| acc + (x - median).abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_moves2(vec![1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(16, Solution::min_moves2(vec![1, 10, 2, 9]));
    }
}

// 561. Array Partition I
// https://leetcode.com/problems/array-partition-i/

use crate::Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::array_pair_sum(vec![1, 4, 3, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(9, Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]));
    }
}

// 718. Maximum Length of Repeated Subarray
// https://leetcode.com/problems/maximum-length-of-repeated-subarray/

use crate::Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![0; nums1.len() + 1]; nums2.len() + 1];
        for (i, n1) in nums1.iter().enumerate().rev() {
            for (j, n2) in nums2.iter().enumerate().rev() {
                if n1 == n2 {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                    res = res.max(dp[i][j]);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]),
            5
        );
    }
}

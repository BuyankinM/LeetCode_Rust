// 2369. Check if There is a Valid Partition For The Array
// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/

use crate::Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let eq_pair = nums[0] == nums[1];
        if nums.len() == 2 {
            return eq_pair;
        }

        let mut dp = vec![false; nums.len() + 1];
        dp[0] = true;
        dp[1] = false;
        dp[2] = eq_pair;

        for (v, i) in nums.windows(3).zip(3..) {
            if let &[a, b, c] = v {
                dp[i] = dp[i - 2] && b == c
                    || dp[i - 3] && (a == b && b == c || b - a == 1 && c - b == 1);

                if !dp[i - 3..i].iter().any(|&x| x) {
                    break;
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::valid_partition(vec![4, 4, 4, 5, 6]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2]));
    }
}

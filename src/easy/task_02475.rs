// 2475. Number of Unequal Triplets in Array
// https://leetcode.com/problems/number-of-unequal-triplets-in-array/

use crate::Solution;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut res = 0;
        for (i, &a) in nums.iter().enumerate().take(l - 2) {
            for (j, &b) in nums.iter().enumerate().skip(i + 1).take(l - 1) {
                res += nums[j + 1..]
                    .iter()
                    .filter(|&&c| a != b && b != c && c != a)
                    .count();
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
        assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::unequal_triplets(vec![1, 3, 1, 2, 4]), 7);
    }
}

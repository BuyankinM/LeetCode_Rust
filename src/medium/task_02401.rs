// 2401. Longest Nice Subarray
// https://leetcode.com/problems/longest-nice-subarray/

use crate::Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut used = 0;
        let mut left = 0;

        for (right, &n) in nums.iter().enumerate() {
            while used & n != 0 {
                // shrink window
                used ^= nums[left];
                left += 1;
            }
            used |= n;
            res = res.max(right - left + 1);
        }

        res as _
    }

    // https://leetcode.com/problems/longest-nice-subarray/discuss/2527586/Rust-or-Two-Pointers-or-With-Comments
    pub fn longest_nice_subarray_v2(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut max_len = 1;
        let mut mask = nums[0];
        for (right, num) in nums.iter().enumerate().skip(1) {
            while *num & mask != 0 {
                mask ^= nums[left];
                left += 1;
            }
            mask |= *num;
            max_len = (right - left + 1).max(max_len);
        }
        max_len as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
    }
}

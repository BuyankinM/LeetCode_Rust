// 300. Longest Increasing Subsequence
// https://leetcode.com/problems/longest-increasing-subsequence/

use crate::Solution;

impl Solution {
    // top-down
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        fn helper(nums: &[i32], dp: &mut [i32], i: usize, prev_i: i32) -> i32 {
            if i == nums.len() {
                return 0;
            }

            let cur_i = (prev_i + 1) as usize;
            if dp[cur_i] != -1 {
                return dp[cur_i];
            }

            let mut include = 0;
            let exclude = helper(nums, dp, i + 1, prev_i);

            if prev_i == -1 || nums[i] > nums[prev_i as usize] {
                include = 1 + helper(nums, dp, i + 1, i as i32);
            }

            dp[cur_i] = include.max(exclude);
            dp[cur_i]
        }

        let mut dp = [-1; 2501];
        helper(&nums, &mut dp, 0, -1)
    }

    // bottom-up
    pub fn length_of_lis_v2(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut ans = 0;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                    ans = ans.max(dp[i]);
                }
            }
        }
        ans
    }

    // https://leetcode.com/problems/longest-increasing-subsequence/discuss/1137421/Rust-solution
    pub fn length_of_lis_patience_sort(nums: Vec<i32>) -> i32 {
        let mut piles: Vec<i32> = vec![];
        for num in nums {
            if let Err(i) = piles.binary_search(&num) {
                match i < piles.len() {
                    true => piles[i] = num,
                    false => piles.push(num),
                }
            }
        }
        piles.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]));
    }
}

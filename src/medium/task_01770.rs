// 1770. Maximum Score from Performing Multiplication Operations
// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/

use crate::Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![0; m + 1];

        for op in (0..m).rev() {
            let next_row = dp.clone();
            for left in (0..=op).rev() {
                let v1 = multipliers[op] * nums[left] + next_row[left + 1];
                let v2 = multipliers[op] * nums[n - 1 - (op - left)] + next_row[left];
                dp[left] = v1.max(v2);
            }
        }
        dp[0]
    }

    // https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/discuss/2581689/Rust-oror-Inspired-by-Pascal's-triangle-oror-Linear-space-quadratic-time
    pub fn maximum_score_v2(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        // Intuition: think of Pascal's triangle and flipping coins.
        // While the combinations blow up as O(2^n), the space blows up as O(n).
        // Let (L,R) be the number of elements taken from each end of `nums`.
        // (This is analogous to (heads,tails) when flipping coins.)
        // For any given (L,R) pair, there exists a maximum score.
        // These are the "stages" like in the travelling stagecoach problem.
        let m = multipliers.len();
        let n = nums.len();
        let mut current: Vec<i32> = vec![0; m + 1];
        let mut future: Vec<i32> = vec![0; m + 1];
        // For each row in Pascal's triangle:
        for (idx, coeff) in multipliers.into_iter().enumerate().rev() {
            // For each cell along that row:
            for l in (0..=idx).rev() {
                let cand_l = current[l + 1] + nums[l] * coeff;
                let cand_r = current[l] + nums[n - 1 - idx + l] * coeff;
                // Which candidate has the better score for the stage?
                future[l] = cand_l.max(cand_r);
            }
            current[0..=idx].copy_from_slice(&future[0..=idx]);
        }
        current[0]
    }

    // https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/discuss/2455163/Rust-or-Bottom-Up-and-Top-Down-DP-or-With-Comments
    pub fn maximum_score_v3(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp_prev = vec![0; m + 1];
        let mut dp_curr = dp_prev.clone();
        for left in (0..m).rev() {
            for right in (0..m - left).rev() {
                let multiplier = multipliers[left + right];
                dp_curr[right] = (dp_prev[right] + multiplier * nums[left])
                    .max(dp_curr[right + 1] + multiplier * nums[n - right - 1]);
            }
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }
        dp_prev[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(14, Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            102,
            Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6])
        );
    }
}

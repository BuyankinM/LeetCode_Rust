// 2370. Longest Ideal Subsequence
// https://leetcode.com/problems/longest-ideal-subsequence/

use crate::Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        if k == 25 {
            return s.len() as i32;
        }

        let ku = k as usize;
        let mut dp = [0; 26];

        for b in s.bytes() {
            let idx = (b - b'a') as usize;
            let (start, end) = (idx.saturating_sub(ku), 25.min(idx + ku));
            dp[idx] = *dp[start..=end].iter().max().unwrap() + 1;
        }
        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_ideal_string("abcd".to_string(), 3), 4);
    }
}

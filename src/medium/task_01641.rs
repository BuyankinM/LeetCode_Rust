// 1641. Count Sorted Vowel Strings
// https://leetcode.com/problems/count-sorted-vowel-strings/

use crate::Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = [1, 1, 1, 1, 1];
        for _ in 1..n {
            for i in 0..4 {
                dp[i] = dp[i..].iter().sum();
            }
        }
        dp.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}

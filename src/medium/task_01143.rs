// 1143. Longest Common Subsequence
// https://leetcode.com/problems/longest-common-subsequence/

use crate::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (l1, l2) = (text1.len(), text2.len());
        let mut dp: Vec<Vec<usize>> = vec![vec![0; l2 + 1]; l1 + 1];
        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                dp[i + 1][j + 1] = match c1 == c2 {
                    true => dp[i][j] + 1,
                    false => std::cmp::max(dp[i][j + 1], dp[i + 1][j]),
                }
            }
        }
        dp[l1][l2] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abcde".to_owned(), "ace".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abc".to_owned(), "abc".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::longest_common_subsequence("abc".to_owned(), "def".to_owned())
        );
    }
}

// 2414. Length of the Longest Alphabetical Continuous Substring
// https://leetcode.com/problems/length-of-the-longest-alphabetical-continuous-substring/

use crate::Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        s.bytes()
            .zip(s[1..].bytes())
            .fold((1, 1), |(l_max, l), (b1, b2)| match b2.saturating_sub(b1) {
                1 => (l_max.max(l + 1), l + 1),
                _ => (l_max, 1),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_continuous_substring("abacaba".to_string()),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_continuous_substring("abcde".to_string()),
            5
        );
    }
}

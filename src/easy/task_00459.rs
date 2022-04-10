// 459. Repeated Substring Pattern
// https://leetcode.com/problems/repeated-substring-pattern/

use crate::Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s2 = s.clone() + s.as_str();
        s2[1..s2.len() - 1].contains(s.as_str())
    }

    // https://leetcode.com/problems/repeated-substring-pattern/discuss/1638208/rust-one-liner
    pub fn repeated_substring_pattern_short(s: String) -> bool {
        s.repeat(2)[1..2 * s.len() - 1].contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }
}

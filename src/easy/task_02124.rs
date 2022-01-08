// 2124. Check if All A's Appears Before All B's
// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/

use crate::Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let (mut ind_a, mut ind_b) = (0, usize::MAX);
        for (ind, c) in s.char_indices() {
            match c {
                'a' => ind_a = ind,
                'b' if ind_b == usize::MAX => ind_b = ind,
                _ => (),
            };
            if ind_a > ind_b {
                return false;
            }
        }
        true
    }

    pub fn check_string_short(s: String) -> bool {
        !s.contains("ba")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_string("aaabbb".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_string("abab".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::check_string("bbb".to_string()));
    }

    #[test]
    fn test_4() {
        assert!(Solution::check_string("aaa".to_string()));
    }
}

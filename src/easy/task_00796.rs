// 796. Rotate String
// https://leetcode.com/problems/rotate-string/

use crate::Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && goal.repeat(2).contains(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::rotate_string("aa".to_string(), "a".to_string()));
    }
}

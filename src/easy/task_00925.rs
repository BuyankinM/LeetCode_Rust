// 925. Long Pressed Name
// https://leetcode.com/problems/long-pressed-name/

use crate::Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let nb = name.as_bytes();
        let mut i = 0;
        for &b in typed.as_bytes() {
            if i < name.len() && b == nb[i] {
                i += 1;
            } else if b != nb[i.saturating_sub(1)] {
                return false;
            }
        }
        i == name.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_long_pressed_name(
            "alex".to_string(),
            "aaleex".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_long_pressed_name(
            "saeed".to_string(),
            "ssaaedd".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_long_pressed_name(
            "leelee".to_string(),
            "lleeelee".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_long_pressed_name(
            "laiden".to_string(),
            "laiden".to_string()
        ));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::is_long_pressed_name(
            "a".to_string(),
            "b".to_string()
        ));
    }

    #[test]
    fn test_6() {
        assert!(!Solution::is_long_pressed_name(
            "alexd".to_string(),
            "ale".to_string()
        ));
    }
}

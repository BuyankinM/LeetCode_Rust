// 520. Detect Capital
// https://leetcode.com/problems/detect-capital/

use crate::Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let (mut upper, mut lower) = (0, 0);
        for (i, c) in word.char_indices() {
            match c.is_ascii_uppercase() {
                true => {
                    upper += 1;
                    if upper == 1 && i > 0 {
                        return false;
                    }
                }
                false => lower += 1,
            }
            if lower > 0 && upper > 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::detect_capital_use("USA".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::detect_capital_use("FlaG".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::detect_capital_use("leetcode".to_string()));
    }

    #[test]
    fn test_4() {
        assert!(Solution::detect_capital_use("Google".to_string()));
    }
}

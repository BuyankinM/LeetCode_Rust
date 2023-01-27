// 2546. Apply Bitwise Operations to Make Strings Equal
// https://leetcode.com/problems/apply-bitwise-operations-to-make-strings-equal/

use crate::Solution;

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s.bytes().any(|b| b == b'1') == target.bytes().any(|b| b == b'1')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::make_strings_equal(
            "1010".to_string(),
            "0110".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::make_strings_equal(
            "11".to_string(),
            "00".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::make_strings_equal(
            "0".to_string(),
            "0".to_string()
        ));
    }
}

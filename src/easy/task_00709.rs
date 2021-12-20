// 709. To Lower Case
// https://leetcode.com/problems/to-lower-case/

use crate::Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }

    // https://leetcode.com/problems/to-lower-case/discuss/461255/One-Line-Rust-Solution-Without-Cheating
    pub fn to_lower_case_no_cheat(str: String) -> String {
        str.chars().map(|c| ((c as u8) | 32) as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "hello".to_string(),
            Solution::to_lower_case("Hello".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "here".to_string(),
            Solution::to_lower_case("here".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "lovely".to_string(),
            Solution::to_lower_case("LOVELY".to_string())
        );
    }
}

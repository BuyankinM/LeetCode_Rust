// 521. Longest Uncommon Subsequence I
// https://leetcode.com/problems/longest-uncommon-subsequence-i/

use crate::Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        use std::cmp::Ordering::{Equal, Greater, Less};
        let (la, lb) = (a.len() as i32, b.len() as i32);
        match la.cmp(&lb) {
            Greater => la,
            Less => lb,
            Equal if a != b => la,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::find_lu_slength("aba".to_string(), "cdc".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::find_lu_slength("aaa".to_string(), "bbb".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::find_lu_slength("aaa".to_string(), "aaa".to_string())
        );
    }
}

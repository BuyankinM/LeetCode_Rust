// 680. Valid Palindrome II
// https://leetcode.com/problems/valid-palindrome-ii/

use crate::Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let sb = s.as_bytes();
        let mut num_drop = 0;
        let (mut i, mut j) = (0, s.len() - 1);
        let (mut i_drop, mut j_drop) = (0, 0);
        while i < j {
            if sb[i] != sb[j] {
                if num_drop == 2 {
                    return false;
                } else if num_drop == 0 {
                    i_drop = i;
                    j_drop = j;
                    i += 1;
                } else {
                    i = i_drop;
                    j = j_drop;
                    j -= 1;
                }
                num_drop += 1;
            } else {
                i += 1;
                j -= 1;
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
        assert!(Solution::valid_palindrome("aba".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::valid_palindrome("abca".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::valid_palindrome("abc".to_string()));
    }

    #[test]
    fn test_4() {
        let s = "aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string();
        assert!(Solution::valid_palindrome(s));
    }

    #[test]
    fn test_5() {
        assert!(Solution::valid_palindrome("acxcybycxcxa".to_string()));
    }
}

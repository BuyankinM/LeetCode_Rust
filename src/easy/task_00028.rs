// 28. Implement strStr()
// https://leetcode.com/problems/implement-strstr/

use crate::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (hl, nl) = (haystack.len(), needle.len());

        if nl == 0 {
            return 0;
        } else if nl > hl {
            return -1;
        }

        for i in 0..=(hl - nl) {
            if haystack[i..]
                .as_bytes()
                .iter()
                .zip(needle.as_bytes().iter())
                .all(|(b1, b2)| b1 == b2)
            {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::str_str("hello".to_owned(), "ll".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::str_str("aaaaa".to_owned(), "bba".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::str_str("".to_owned(), "".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(-1, Solution::str_str("".to_owned(), "a".to_owned()));
    }
}

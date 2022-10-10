// 1328. Break a Palindrome
// https://leetcode.com/problems/break-a-palindrome/

use crate::Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        match palindrome.len() {
            1 => "".to_string(),
            l => {
                let mut is_find = false;
                let mut pb = palindrome.into_bytes();
                for b in pb.iter_mut().take(l / 2) {
                    if *b > b'a' {
                        *b = b'a';
                        is_find = true;
                        break;
                    }
                }
                if !is_find {
                    // all 'a' in half-string
                    if let Some(b) = pb.last_mut() {
                        *b = b'b';
                    }
                }
                String::from_utf8(pb).unwrap()
            }
        }
    }

    // https://leetcode.com/problems/break-a-palindrome/discuss/2684927/rust-greedy-with-comments
    pub fn break_palindrome_func(palindrome: String) -> String {
        let n = palindrome.len();
        if n < 2 {
            String::new()
        } else {
            let mut palindrome = palindrome.as_bytes().to_vec();
            match palindrome.iter_mut().take(n / 2).find(|b| **b != b'a') {
                Some(b) => *b = b'a',
                None => *palindrome.last_mut().unwrap() = b'b',
            }
            palindrome.into_iter().map(|b| b as char).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::break_palindrome("abccba".to_string()),
            "aaccba".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::break_palindrome("a".to_string()), "".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::break_palindrome("aba".to_string()),
            "abb".to_string()
        );
    }
}

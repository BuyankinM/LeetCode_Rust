// 859. Buddy Strings
// https://leetcode.com/problems/buddy-strings/

use crate::Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let mut counter = [0; 26];
        let mut swap_flag = 0;
        let mut swap_pair = (0, 0);

        for (&b1, &b2) in s.as_bytes().iter().zip(goal.as_bytes().iter()) {
            if b1 != b2 {
                match swap_flag {
                    0 => {
                        swap_pair = (b1, b2);
                        swap_flag = 1
                    }
                    1 if swap_pair.0 == b2 && swap_pair.1 == b1 => swap_flag = 2,
                    _ => return false,
                }
            }
            counter[(b1 - b'a') as usize] += 1;
        }

        swap_flag == 2 || swap_flag == 0 && counter.iter().any(|n| *n > 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::buddy_strings("ab".to_owned(), "ab".to_owned()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::buddy_strings("ab".to_owned(), "ba".to_owned()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::buddy_strings("aa".to_owned(), "aa".to_owned()));
    }

    #[test]
    fn test_4() {
        assert!(Solution::buddy_strings(
            "aaaaaaabc".to_owned(),
            "aaaaaaacb".to_owned()
        ));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::buddy_strings(
            "abac".to_owned(),
            "abad".to_owned()
        ));
    }
}

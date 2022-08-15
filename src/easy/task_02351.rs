// 2351. First Letter to Appear Twice
// https://leetcode.com/problems/first-letter-to-appear-twice/

use crate::Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut counter = [0; 26];
        for b in s.bytes() {
            let n = &mut counter[(b - b'a') as usize];
            match *n == 1 {
                true => return b as char,
                false => *n += 1,
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }
}

// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut positions = [0; 128];
        let (mut max_len, mut start) = (0, 1);

        for (b, i) in s.bytes().zip(1..) {
            let prev_pos = &mut positions[(b - b' ') as usize];
            match *prev_pos {
                0 => *prev_pos = i,
                _ => {
                    max_len = max_len.max(i - start);
                    start = start.max(*prev_pos + 1);
                    *prev_pos = i;
                }
            }
        }

        max_len.max(s.len() as i32 - start + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(2, Solution::length_of_longest_substring("abba".to_string()));
    }
}

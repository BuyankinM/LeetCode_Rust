// 2609. Find the Longest Balanced Substring of a Binary String
// https://leetcode.com/problems/find-the-longest-balanced-substring-of-a-binary-string/

use crate::Solution;

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let len_s = |res: i32, z: i32, o: i32| res.max(2 * z.min(o));
        let (mut res, mut n_zeros, mut n_ones) = (0, 0, 0);
        let mut prev_b = b' ';
        for b in s.bytes() {
            match b == b'0' {
                true => {
                    if prev_b != b {
                        res = len_s(res, n_zeros, n_ones);
                        n_zeros = 0;
                        n_ones = 0;
                    }
                    n_zeros += 1;
                }
                false => n_ones += 1,
            }
            prev_b = b;
        }
        len_s(res, n_zeros, n_ones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::find_the_longest_balanced_substring("01000111".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::find_the_longest_balanced_substring("00111".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::find_the_longest_balanced_substring("111".to_string())
        );
    }
}

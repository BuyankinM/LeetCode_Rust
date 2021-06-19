// 1624. Largest Substring Between Two Equal Characters
// https://leetcode.com/problems/largest-substring-between-two-equal-characters/

use crate::Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut ar = [-1; 26];
        let mut max_len = -1;
        for (ind, &b) in s.as_bytes().iter().enumerate() {
            let ind_b = (b - b'a') as usize;
            match ar[ind_b] == -1 {
                true => ar[ind_b] = ind as i32,
                false => max_len = max_len.max((ind as i32) - ar[ind_b] - 1),
            }
        }
        max_len
    }

    fn max_length_between_equal_characters_zero_based(s: String) -> i32 {
        let mut ar = [0; 26];
        let mut max_len = 0;
        for (ind, &b) in s.as_bytes().iter().enumerate() {
            let ind_b = (b - b'a') as usize;
            match ar[ind_b] {
                0 => ar[ind_b] = ind + 1,
                prev_ind => max_len = max_len.max(ind - prev_ind + 1),
            }
        }
        max_len as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            0,
            Solution::max_length_between_equal_characters("aa".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::max_length_between_equal_characters("abca".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::max_length_between_equal_characters_zero_based("cbzxy".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            4,
            Solution::max_length_between_equal_characters_zero_based("cabbac".to_owned())
        );
    }
}

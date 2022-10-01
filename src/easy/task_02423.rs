// 2423. Remove Letter To Equalize Frequency
// https://leetcode.com/problems/remove-letter-to-equalize-frequency/

use crate::Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut counter = [0; 26];
        let (mut min_val, mut max_val) = (i32::MAX, i32::MIN);

        word.bytes().for_each(|b| {
            let n = &mut counter[(b - b'a') as usize];
            *n += 1;
            max_val = max_val.max(*n);
        });

        let (mut num_max, mut total) = (0, 0);
        counter.iter().filter(|n| **n > 0).for_each(|&n| {
            if n == max_val {
                num_max += 1;
            }
            min_val = min_val.min(n);
            total += 1;
        });

        // 4 possible options:
        // aaaaa || abcde
        // abcdd
        // aabcc
        num_max == total && (total == 1 || max_val == 1)
            || num_max == 1 && (max_val - min_val) == 1
            || num_max == total - 1 && min_val == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::equal_frequency("aaaaa".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::equal_frequency("abcde".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::equal_frequency("abcdd".to_string()));
    }

    #[test]
    fn test_4() {
        assert!(Solution::equal_frequency("aabcc".to_string()));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::equal_frequency("aacc".to_string()));
    }
}

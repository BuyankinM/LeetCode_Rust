// 1880. Check if Word Equals Summation of Two Words
// https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/

use crate::Solution;

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let calc_num_str = |s: String| {
            s.as_bytes()
                .iter()
                .enumerate()
                .fold(0, |acc, (ind, b)| -> u32 {
                    acc + (b - b'a') as u32 * 10u32.pow((s.len() - ind - 1) as u32)
                })
        };
        calc_num_str(first_word) + calc_num_str(second_word) == calc_num_str(target_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_sum_equal(
            "acb".to_owned(),
            "cba".to_owned(),
            "cdb".to_owned()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_sum_equal(
            "aaa".to_owned(),
            "a".to_owned(),
            "aab".to_owned()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_sum_equal(
            "aaa".to_owned(),
            "a".to_owned(),
            "aaaa".to_owned()
        ));
    }
}

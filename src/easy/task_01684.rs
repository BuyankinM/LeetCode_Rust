// 1684. Count the Number of Consistent Strings
// https://leetcode.com/problems/count-the-number-of-consistent-strings/

use crate::Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mask = allowed.as_bytes().iter().fold(0, |mut acc, &c| {
            acc += 1 << (c - b'a');
            acc
        });

        let mut result = 0;
        for word in words {
            if word
                .as_bytes()
                .iter()
                .all(|&c| (mask & (1 << (c - b'a'))) > 0)
            {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::count_consistent_strings(
                "ab".to_owned(),
                vec![
                    "ad".to_owned(),
                    "bd".to_owned(),
                    "aaab".to_owned(),
                    "baa".to_owned(),
                    "badab".to_owned()
                ]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            7,
            Solution::count_consistent_strings(
                "abc".to_owned(),
                vec![
                    "a".to_owned(),
                    "b".to_owned(),
                    "c".to_owned(),
                    "ab".to_owned(),
                    "ac".to_owned(),
                    "bc".to_owned(),
                    "abc".to_owned()
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::count_consistent_strings(
                "cad".to_owned(),
                vec![
                    "cc".to_owned(),
                    "acd".to_owned(),
                    "b".to_owned(),
                    "ba".to_owned(),
                    "bac".to_owned(),
                    "bad".to_owned(),
                    "ac".to_owned(),
                    "d".to_owned()
                ]
            )
        );
    }
}

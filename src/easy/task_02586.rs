// 2586. Count the Number of Vowel Strings in Range
// https://leetcode.com/problems/count-the-number-of-vowel-strings-in-range/

use crate::Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words
            .iter()
            .skip(left as usize)
            .take((right - left + 1) as usize)
            .filter(|s| {
                s.starts_with(['a', 'e', 'i', 'o', 'u']) && s.ends_with(['a', 'e', 'i', 'o', 'u'])
            })
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::vowel_strings(
                vec!["are".to_string(), "amy".to_string(), "u".to_string()],
                0,
                2
            )
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::vowel_strings(
                vec![
                    "hey".to_string(),
                    "aeo".to_string(),
                    "mu".to_string(),
                    "ooo".to_string(),
                    "artro".to_string()
                ],
                1,
                4
            )
        )
    }
}

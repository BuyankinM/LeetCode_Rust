// 1935. Maximum Number of Words You Can Type
// https://leetcode.com/problems/maximum-number-of-words-you-can-type/

use crate::Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_hash = broken_letters
            .into_bytes()
            .iter()
            .fold([false; 27], |mut acc, &b| {
                acc[(b - b'a') as usize] = true;
                acc
            });

        text.split_ascii_whitespace()
            .filter(|&s| {
                s.as_bytes()
                    .iter()
                    .all(|&b| !broken_hash[(b - b'a') as usize])
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
            1,
            Solution::can_be_typed_words("hello world".to_owned(), "ad".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::can_be_typed_words("leet code".to_owned(), "lt".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::can_be_typed_words("leet code".to_owned(), "e".to_owned())
        );
    }
}

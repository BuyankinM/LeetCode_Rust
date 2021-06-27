// 1832. Check if the Sentence Is Pangram
// https://leetcode.com/problems/check-if-the-sentence-is-pangram/

use crate::Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence
            .as_bytes()
            .iter()
            .fold([false; 26], |mut count_arr, &x| {
                count_arr[(x - b'a') as usize] = true;
                count_arr
            })
            .iter()
            .all(|&x| x)
    }

    pub fn check_if_pangram_short(sentence: String) -> bool {
        use std::collections::HashSet;
        sentence.chars().collect::<HashSet<_>>().len() == 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            false,
            Solution::check_if_pangram_short("leetcode".to_owned())
        );
    }
}

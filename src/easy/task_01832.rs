// 1832. Check if the Sentence Is Pangram
// https://leetcode.com/problems/check-if-the-sentence-is-pangram/

use crate::Solution;

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

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

    // https://leetcode.com/problems/check-if-the-sentence-is-pangram/discuss/2713116/Rust-or-Bit-Set-or-With-Comments
    pub fn check_if_pangram_optimal(sentence: String) -> bool {
        sentence
            .bytes()
            .scan(0_u32, |bitset, b| {
                *bitset |= 1 << (b - b'a');
                Some(*bitset)
            })
            .any(|bitset| bitset == (1 << N_LETTERS) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_owned()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_if_pangram_short("leetcode".to_owned()));
    }
}

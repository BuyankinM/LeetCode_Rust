// 2047. Number of Valid Words in a Sentence
// https://leetcode.com/problems/number-of-valid-words-in-a-sentence/

use crate::Solution;

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut res = 0;
        'outer: for word in sentence.split_ascii_whitespace() {
            let l = word.len();
            let mut find_hyphen = false;
            let mut find_punct = false;
            let mut check_next_alpha = false;

            for (ind, c) in word.chars().enumerate() {
                let is_hyphen = c == '-';
                let is_punct = "!.,".contains(c);

                if c.is_ascii_digit()
                    || (is_hyphen && (find_hyphen || ind == 0 || ind == l - 1))
                    || (is_punct && (find_punct || ind != l - 1))
                    || (check_next_alpha && !c.is_alphabetic())
                {
                    continue 'outer;
                } else if is_hyphen {
                    find_hyphen = true;
                    check_next_alpha = true;
                } else if is_punct {
                    find_punct = true;
                } else if check_next_alpha {
                    check_next_alpha = false;
                }
            }
            res += 1;
        }
        res
    }
}

// 1668. Maximum Repeating Substring
// https://leetcode.com/problems/maximum-repeating-substring/

use crate::Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut max_count = 0;
        let mut count;

        let (sb, wb) = (sequence.as_bytes(), word.as_bytes());
        let l = word.len();
        let mut i = 0;

        while i < sequence.len() {
            count = 0;
            for j in i..sequence.len() {
                match sb[j] == wb[count % l] {
                    true => count += 1,
                    false => break,
                }
            }
            let n = count / l;
            if n > 0 {
                max_count = max_count.max(n);
                if n > 1 {
                    i += l * (n - 1) - 1; // jump to (n-1) coincidence
                }
            }
            i += 1;
        }

        max_count as i32
    }

    pub fn max_repeating_short(sequence: String, word: String) -> i32 {
        let max_word = word.repeat(sequence.len() / word.len());
        let mut n = max_word.len();
        while sequence.find(&max_word[..n]).is_none() {
            n -= word.len();
        }
        (n / word.len()) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::max_repeating("ababc".to_owned(), "ab".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::max_repeating("ababc".to_owned(), "ba".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::max_repeating_short("ababc".to_owned(), "ac".to_owned())
        );
    }
}

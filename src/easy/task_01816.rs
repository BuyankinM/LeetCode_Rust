// 1816. Truncate Sentence
// https://leetcode.com/problems/truncate-sentence/

use crate::Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        (&s).splitn((k + 1) as usize, ' ')
            .take(k as usize)
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Hello how are you".to_owned(),
            Solution::truncate_sentence("Hello how are you Contestant".to_owned(), 4)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "What is the solution".to_owned(),
            Solution::truncate_sentence("What is the solution to this problem".to_owned(), 4)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "chopper is not a tanuki".to_owned(),
            Solution::truncate_sentence("chopper is not a tanuki".to_owned(), 5)
        );
    }
}

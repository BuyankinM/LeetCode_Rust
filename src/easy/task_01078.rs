// 1078. Occurrences After Bigram
// https://leetcode.com/problems/occurrences-after-bigram/

use crate::Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut counter = 0u8;
        let mut res = Vec::new();
        let mut prev_ind = 0;
        let eq_strings = first == second;

        for (ind, word) in text.split(' ').enumerate() {
            if counter == 0b11 && (ind - prev_ind == 1) {
                res.push(word.to_string());
                counter = (eq_strings && word == first) as u8;
            }

            match (counter, word == first, word == second) {
                (_, true, false) | (0, true, true) => counter = 0b01,
                (_, _, true) if ind - prev_ind == 1 => counter |= 0b10,
                _ => continue,
            }

            prev_ind = ind;
        }
        res
    }

    // https://leetcode.com/problems/occurrences-after-bigram/discuss/1262269/Rust-Using-Iterator%3A%3Ascan-0ms-2MB
    pub fn find_ocurrences_func(text: String, first: String, second: String) -> Vec<String> {
        text.split_whitespace()
            .scan(("", ""), |(q, r), w| {
                let p = *q;
                *q = r;
                *r = w;
                Some(if p == first && q == &second {
                    Some(w.to_string())
                } else {
                    None
                })
            })
            .flatten()
            .collect()
    }

    // https://leetcode.com/problems/occurrences-after-bigram/discuss/1157068/Rust-100-speed-and-memory
    pub fn find_ocurrences_iter(text: String, first: String, second: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut iter = text.split_whitespace().peekable();
        let mut next = iter.next();

        while let Some(word) = next {
            if word == first && iter.peek() == Some(&second.as_str()) {
                next = iter.next();
                if let Some(w) = iter.peek() {
                    ans.push(w.to_string());
                }
                continue;
            }
            next = iter.next();
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["girl".to_string(), "student".to_string()],
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".to_string(),
                "a".to_string(),
                "good".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["we".to_string(), "rock".to_string()],
            Solution::find_ocurrences(
                "we will we will rock you".to_string(),
                "we".to_string(),
                "will".to_string()
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![
                "we".to_string(),
                "a".to_string(),
                "b".to_string(),
                "we".to_string()
            ],
            Solution::find_ocurrences(
                "we we we a we we b we we we".to_string(),
                "we".to_string(),
                "we".to_string()
            )
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![
                "we".to_string(),
                "a".to_string(),
                "b".to_string(),
                "we".to_string()
            ],
            Solution::find_ocurrences_func(
                "we we we a we we b we we we".to_string(),
                "we".to_string(),
                "we".to_string()
            )
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            vec![
                "we".to_string(),
                "a".to_string(),
                "b".to_string(),
                "we".to_string()
            ],
            Solution::find_ocurrences_iter(
                "we we we a we we b we we we".to_string(),
                "we".to_string(),
                "we".to_string()
            )
        );
    }
}

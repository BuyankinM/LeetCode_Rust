// 820. Short Encoding of Words
// https://leetcode.com/problems/short-encoding-of-words/
use crate::Solution;

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        let mut res_string = String::new();

        words.sort_unstable_by(|s1, s2| s2.len().partial_cmp(&s1.len()).unwrap());

        for word in words.into_iter().map(|s| s + "#") {
            match res_string.contains(&word) {
                true => continue,
                false => res_string += &word,
            };
        }
        res_string.len() as i32
    }

    pub fn minimum_length_encoding_hashset(words: Vec<String>) -> i32 {
        let mut ignore: std::collections::HashSet<_> = words
            .iter()
            .flat_map(|w| w.char_indices().skip(1).map(move |(i, _)| &w[i..]))
            .collect();

        words
            .iter()
            .filter_map(|w| Some(w.len() as i32 + 1).filter(|_| ignore.insert(w)))
            .sum()
    }

    pub fn minimum_length_encoding_best(words: Vec<String>) -> i32 {
        let mut v = words
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>();

        v.sort_unstable();

        let mut answer = 0;
        for (i, word) in v.iter().enumerate() {
            if i + 1 >= v.len() || !v[i + 1].starts_with(&v[i]) {
                answer += word.len() + 1;
            }
        }
        answer as i32
    }
}

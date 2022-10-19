// 692. Top K Frequent Words
// https://leetcode.com/problems/top-k-frequent-words/

use crate::Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut hm = std::collections::HashMap::with_capacity(words.len());
        words
            .into_iter()
            .for_each(|w| *hm.entry(w).or_insert(0) += 1);

        let mut res = hm.into_iter().collect::<Vec<_>>();
        res.sort_unstable_by(|(w1, n1), (w2, n2)| n2.cmp(n1).then(w1.cmp(w2)));
        res.truncate(k as usize);
        res.into_iter().map(|(word, _)| word).collect()
    }

    pub fn top_k_frequent_heap(words: Vec<String>, k: i32) -> Vec<String> {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        let mut hm = HashMap::with_capacity(words.len());
        words
            .into_iter()
            .for_each(|w| *hm.entry(w).or_insert(0) += 1);

        let mut heap = BinaryHeap::with_capacity(hm.len());
        hm.into_iter()
            .for_each(|(word, num)| heap.push((num, Reverse(word))));

        let mut res = Vec::with_capacity(k as usize);
        (0..k).for_each(|_| {
            if let Some((_, Reverse(word))) = heap.pop() {
                res.push(word);
            }
        });
        res
    }

    // https://leetcode.com/problems/top-k-frequent-words/discuss/2721072/Rust-or-Hash-Map-%2B-Sort-or-With-Comments
    pub fn top_k_frequent_short(words: Vec<String>, k: i32) -> Vec<String> {
        use std::collections::HashMap;
        let mut freqs = words
            .into_iter()
            .fold(HashMap::<String, usize>::new(), |mut freqs, s| {
                *freqs.entry(s).or_default() += 1;
                freqs
            })
            .into_iter()
            .collect::<Vec<_>>();
        freqs.sort_unstable_by(|(s1, f1), (s2, f2)| f2.cmp(f1).then(s1.cmp(s2)));
        freqs.into_iter().take(k as usize).map(|(s, _)| s).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["i".to_string(), "love".to_string()],
            Solution::top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                "the".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "day".to_string()
            ],
            Solution::top_k_frequent(
                vec![
                    "the".to_string(),
                    "day".to_string(),
                    "is".to_string(),
                    "sunny".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "sunny".to_string(),
                    "is".to_string(),
                    "is".to_string()
                ],
                4
            )
        );
    }
}

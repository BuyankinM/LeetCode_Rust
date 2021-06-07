// 916. Word Subsets
// https://leetcode.com/problems/word-subsets/
use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();

        let get_word_hashmap = |s: &String| {
            let mut mini_hash: HashMap<char, u32> = HashMap::new();
            s.chars()
                .for_each(|ch| *mini_hash.entry(ch).or_insert(0) += 1);
            mini_hash
        };

        let mut full_hash: HashMap<char, u32> = HashMap::new();

        for sb in &b {
            let mini_hash = get_word_hashmap(sb);

            for (ch, num) in mini_hash {
                let ent = full_hash.entry(ch).or_insert(0);
                if *ent < num {
                    *ent = num;
                }
            }
        }

        'main_loop: for sa in &a {
            let mut mini_hash = get_word_hashmap(sa);

            for (ch, num) in &full_hash {
                let ent = mini_hash.entry(*ch).or_insert(0);
                if *ent < *num {
                    continue 'main_loop;
                }
            }

            res.push(sa.to_owned());
        }

        res
    }

    pub fn _word_subsets_fast(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let char_counts = |s: &String| -> [usize; 26] {
            s.as_bytes().iter().fold([0; 26], |mut acc, &u| {
                acc[(u - b'a') as usize] += 1;
                acc
            })
        };
        let target = b.iter().map(char_counts).fold(vec![0; 26], |acc, x| {
            acc.iter()
                .enumerate()
                .map(|(i, &count)| x[i].max(count))
                .collect()
        });
        a.into_iter()
            .filter(|s| {
                char_counts(s)
                    .iter()
                    .enumerate()
                    .all(|(i, &count)| count >= target[i])
            })
            .collect()
    }
}

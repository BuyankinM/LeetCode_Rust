// 49. Group Anagrams
// https://leetcode.com/problems/group-anagrams/

use crate::Solution;

use std::collections::HashMap;

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let get_key = |s: &str| -> Vec<i32> {
            s.bytes().fold(vec![0; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                acc
            })
        };

        let mut hm: HashMap<Vec<i32>, Vec<String>> = HashMap::with_capacity(strs.len());
        strs.into_iter()
            .for_each(|s| hm.entry(get_key(s.as_str())).or_default().push(s));
        hm.into_values().collect()
    }

    // https://leetcode.com/problems/group-anagrams/discuss/2751413/Rust-or-HashMap-or-Functional-Style-or-With-Comments
    pub fn group_anagrams_func(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(
                HashMap::<[u8; N_LETTERS], Vec<String>>::new(),
                |mut map, s| {
                    let freqs = s.bytes().map(|b| (b - b'a') as usize).fold(
                        [0; N_LETTERS],
                        |mut freqs, bin| {
                            freqs[bin] += 1;
                            freqs
                        },
                    );
                    map.entry(freqs).or_default().push(s);
                    map
                },
            )
            .into_values()
            .collect()
    }
}

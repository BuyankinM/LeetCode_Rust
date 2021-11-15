// 2062. Count Vowel Substrings of a String
// https://leetcode.com/problems/count-vowel-substrings-of-a-string/

use std::{collections::btree_map::OccupiedEntry, iter::FromIterator};

use crate::Solution;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        use std::collections::HashSet;

        if word.len() < 5 {
            return 0;
        }

        let wb = word.as_bytes();
        let mut acc = HashSet::with_capacity(5);
        let vowels: HashSet<u8> = "aeiou".chars().map(|c| c as u8).collect();

        let mut res = 0;

        for i in 0..word.len() - 4 {
            for &b in wb[i..].iter().take_while(|b| vowels.contains(*b)) {
                acc.insert(b);
                if acc.len() == 5 {
                    res += 1;
                }
            }
            acc.clear();
        }
        res
    }

    // https://leetcode.com/problems/count-vowel-substrings-of-a-string/discuss/1563737/Sliding-Window
    pub fn count_vowel_substrings_sliding_window(word: String) -> i32 {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let mut res = 0;
        let wb = word.as_bytes();
        let mut map: HashMap<u8, i32> = "aeiou".chars().map(|c| (c as u8, 0)).collect();

        let (mut j, mut k, mut cnt_vow) = (0, 0, 0);
        for (i, &b) in wb.iter().enumerate() {
            match map.entry(b) {
                Entry::Occupied(mut vi) => {
                    let num = vi.get_mut();
                    *num += 1;
                    if *num == 1 {
                        cnt_vow += 1;
                    }

                    while cnt_vow == 5 {
                        if let Entry::Occupied(vk) = map.entry(wb[k]).and_modify(|val| *val -= 1) {
                            if *vk.get() == 0 {
                                cnt_vow -= 1;
                            }
                        }
                        k += 1;
                    }

                    res += k - j;
                }
                _ => {
                    j = i + 1;
                    k = i + 1;
                    cnt_vow = 0;
                    map.iter_mut().for_each(|(_, val)| *val = 0);
                }
            }
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_vowel_substrings("aeiouu".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::count_vowel_substrings("unicornarihan".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            7,
            Solution::count_vowel_substrings("cuaieuouac".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::count_vowel_substrings("a".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            81,
            Solution::count_vowel_substrings("duuebuaeeeeeeuaoeiueaoui".to_string())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            81,
            Solution::count_vowel_substrings_sliding_window("duuebuaeeeeeeuaoeiueaoui".to_string())
        );
    }
}

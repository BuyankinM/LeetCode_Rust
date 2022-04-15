// 387. First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string/

use crate::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let idx = |c: char| (c as u8 - b'a') as usize;
        let mut map = [0; 26];
        s.chars().for_each(|c| map[idx(c)] += 1);

        for (pos, c) in s.char_indices() {
            if map[idx(c)] == 1 {
                return pos as i32;
            }
        }
        -1
    }

    // https://leetcode.com/problems/first-unique-character-in-a-string/discuss/695922/Rust-short'n'sweet
    pub fn first_uniq_char_func(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        s.chars()
            .enumerate()
            .find_map(|(i, c)| {
                if *map.get(&c).unwrap() == 1 {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::first_uniq_char("cc".to_string()), -1);
    }
}

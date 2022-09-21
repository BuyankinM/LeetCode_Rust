// 2405. Optimal Partition of String
// https://leetcode.com/problems/optimal-partition-of-string/

use crate::Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let (mut res, mut mask) = (1, 0);

        s.bytes().for_each(|b| {
            let letter_mask = 1 << (b - b'a');
            match letter_mask & mask == 0 {
                true => mask |= letter_mask,
                false => {
                    res += 1;
                    mask = letter_mask;
                }
            }
        });

        res
    }

    // https://leetcode.com/problems/optimal-partition-of-string/discuss/2560220/Rust-or-Greedy-or-With-Comments
    pub fn partition_string_array(s: String) -> i32 {
        let mut map = [false; 26];
        let mut rez = 1;
        for b in s.bytes().map(|b| (b - b'a') as usize) {
            if map[b] {
                map.fill(false);
                rez += 1;
            }
            map[b] = true;
        }
        rez
    }

    // https://leetcode.com/problems/optimal-partition-of-string/discuss/2560220/Rust-or-Greedy-or-With-Comments
    pub fn partition_string_fold(s: String) -> i32 {
        s.bytes()
            .fold((1, 0), |(rez, arr), b| match 1 << (b - b'a') {
                bit if arr & bit == bit => (rez + 1, bit),
                bit => (rez, arr | bit),
            })
            .0
    }

    // https://leetcode.com/problems/optimal-partition-of-string/discuss/2562947/Rust-0-ms-two-solutions-(with-detailed-comments)
    pub fn partition_string_functional(s: String) -> i32 {
        // [1] use simple array to keep track of unique letters
        let mut letters = [false; 26];

        // [2] count how many times we encountered duplicates
        s.bytes()
            .map(|c| (c - b'a') as usize)
            .filter(|&i| {
                let f = letters[i];
                if f {
                    letters.fill(false);
                }
                letters[i] = true;
                f
            })
            .count() as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::partition_string("abacaba".to_string()), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::partition_string("ssssss".to_string()), 6);
    }
}

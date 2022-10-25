// 1239. Maximum Length of a Concatenated String with Unique Characters
// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/

use crate::Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        fn get_bit_set(s: &str) -> i32 {
            let mut res = 0;
            for b in s.bytes().map(|b| 1 << (b - b'a')) {
                match b & res == 0 {
                    true => res |= b,
                    false => return 0,
                }
            }
            res
        }

        let mut dp = vec![(0, 0)];

        for s in arr.iter() {
            let (cur_mask, cur_l) = (get_bit_set(s), s.len());
            if cur_mask == 0 {
                continue;
            }

            for i in 0..dp.len() {
                let (prev_l, prev_mask) = dp[i];
                if cur_mask & prev_mask == 0 {
                    dp.push((cur_l + prev_l, cur_mask | prev_mask));
                }
            }
        }
        dp.iter().map(|(l, _)| *l).max().unwrap_or_default() as _
    }

    // https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/discuss/2737493/PythonC%2B%2BJavaRust-0-ms-bit-and-set-operations-(with-detailed-comments)
    pub fn max_length_optimal(arr: Vec<String>) -> i32 {
        let mut max_len: u32 = 0;

        // [1] we should first throw away all strings with any
        //    duplicate characters; strings with all unique
        //    characters are the subsets of the alphabet, thus,
        //    can be encoded using binary form
        let mut unique: Vec<u32> = Vec::new();
        for s in arr {
            // here, we set bits for each of encountered letters
            let bin: u32 = s.bytes().map(|c| 1 << (c - b'a')).sum();
            if bin.count_ones() == s.len() as u32 {
                unique.push(bin);
            }
        }

        // [2] now start with an empty concatenation and iteratively
        //    increase its length by trying to add more strings
        let mut concat: Vec<u32> = vec![0];
        for u in unique {
            for i in 0..concat.len() {
                if (concat[i] & u).count_ones() == 0 {
                    concat.push(concat[i] | u);
                    max_len = max_len.max(concat[i].count_ones() + u.count_ones());
                }
            }
        }

        max_len as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            6,
            Solution::max_length(vec![
                "cha".to_string(),
                "r".to_string(),
                "act".to_string(),
                "ers".to_string()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            26,
            Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            2,
            Solution::max_length(vec!["uun".to_string(), "iiq".to_string(), "ue".to_string()])
        );
    }
}

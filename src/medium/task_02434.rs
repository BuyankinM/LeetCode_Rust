// 2434. Using a Robot to Print the Lexicographically Smallest String
// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/

use crate::Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        // collect all last positions of chars in sorted order
        let mut map = std::collections::BTreeMap::new();
        s.char_indices().for_each(|(i, c)| {
            map.insert(c, i as i32);
        });

        let mut res = String::with_capacity(s.len());
        let mut t = Vec::with_capacity(s.len());
        let mut prev_i = -1;

        for (c, i) in map.into_iter() {
            // check top of stack
            while let Some(&ct) = t.last() {
                match ct <= c {
                    true => res.push(t.pop().unwrap()),
                    false => break,
                }
            }

            if i > prev_i {
                // go to last next char
                let start = (prev_i + 1) as usize;
                let n = (i - prev_i) as usize;

                s[start..].chars().take(n).for_each(|cc| match c == cc {
                    true => res.push(c),
                    false => t.push(cc),
                });

                prev_i = i;
            }
        }

        // other symbols from stack
        res.extend(t.iter().cloned().rev());
        res
    }

    // https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/discuss/2692045/Rust-2-ms-fastest-(100)-overoptimized-solution-%2B-bonus-(with-detailed-comments)
    pub fn robot_with_string_fast(s: String) -> String {
        let mut paper: Vec<u8> = Vec::with_capacity(s.len());
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        let bytes = s.as_bytes();

        // [1] determine the position of last occurence for each letter;
        //     for absent letters, the value is usize::MAX
        let mut last_pos: [usize; 26] = [usize::MAX; 26];
        let mut letters: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        for (pos, ch) in s.bytes().enumerate().rev() {
            let ch = (ch - b'a') as usize;
            if last_pos[ch] == usize::MAX {
                last_pos[ch] = pos;
                letters |= 1 << ch;
            }
            if letters == 0b0000_0011_1111_1111_1111_1111_1111_1111 {
                break;
            }
        }

        // [2] to obtain the lexicographically smallest string,
        //     we always have to print the smallest possible letter;
        //     thus, we first try to print all instances of each
        //     unique letter (see [4]) in the alphabetic order
        let mut lp: usize = 0;
        for (ch, &pos) in last_pos
            .iter()
            .enumerate()
            .filter(|&(_, pos)| *pos < usize::MAX)
        {
            let ch = ch as u8 + b'a';

            // [3] if on some iteration there are letters on top
            //     of the stack that are smaller than the currently
            //     considered letter, move them to the paper
            while !stack.is_empty() && stack[stack.len() - 1] <= ch {
                paper.push(stack.pop().unwrap());
            }

            // [4] main block where we move all instances of each
            //     unique letter to the paper and push all encountered
            //     characters (before the last instance) to the stack
            if lp <= pos {
                bytes[lp..=pos].iter().for_each(|&c| {
                    if c == ch {
                        paper.push(c);
                    } else {
                        stack.push(c);
                    }
                });
                lp = pos + 1;
            }
        }

        String::from_utf8(paper).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::robot_with_string("zza".to_string()),
            "azz".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::robot_with_string("bac".to_string()),
            "abc".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::robot_with_string("bdda".to_string()),
            "addb".to_string()
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::robot_with_string("vzhofnpo".to_string()),
            "fnohopzv".to_string()
        );
    }
}

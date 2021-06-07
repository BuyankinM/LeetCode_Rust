// 647. Palindromic Substrings
// https://leetcode.com/problems/palindromic-substrings/
use crate::Solution;

impl Solution {
    pub fn count_substrings_full_brute_force(s: String) -> i32 {
        let mut n = s.len() as i32;
        let ss = s.as_bytes();

        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                if ss[i..=j]
                    .iter()
                    .zip(ss[i..=j].iter().rev())
                    .all(|(&c1, &c2)| c1 == c2)
                {
                    n += 1
                }
            }
        }

        n
    }

    pub fn count_substrings_half_brute_force(s: String) -> i32 {
        let mut n = s.len() as i32;
        let ss = s.as_bytes();

        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                let med = i + (j - i) / 2;
                if ss[i..=med]
                    .iter()
                    .zip(ss[med + 1..=j].iter().rev())
                    .all(|(&c1, &c2)| c1 == c2)
                {
                    n += 1
                }
            }
        }

        n
    }

    pub fn count_substrings_fast_1(s: String) -> i32 {
        let s = s.into_bytes();
        let count = |i, j| {
            ((0..=i).rev())
                .zip(j..s.len())
                .try_fold(0, |a, (i, j)| if s[i] == s[j] { Ok(a + 1) } else { Err(a) })
                .unwrap_or_else(|a| a)
        };
        (0..s.len()).map(|i| count(i, i) + count(i, i + 1)).sum()
    }

    pub fn count_substrings_fast_my(s: String) -> i32 {
        fn count_palindromes(sb: &Vec<u8>, mut start_1: i32, mut start_2: i32) -> i32 {
            let mut n = 0;
            while start_1 >= 0 && start_2 < sb.len() as i32 {
                if sb[start_1 as usize] != sb[start_2 as usize] {
                    break;
                }

                n += 1;
                start_1 -= 1;
                start_2 += 1;
            }
            n
        }

        let mut n = 0;
        let sb = s.into_bytes();

        for i in 0..sb.len() as i32 {
            n += count_palindromes(&sb, i, i); // abcba
            n += count_palindromes(&sb, i, i + 1); // abba
        }

        n
    }
}

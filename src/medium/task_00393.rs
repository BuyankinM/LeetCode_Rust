// 393. UTF-8 Validation
// https://leetcode.com/problems/utf-8-validation/

use crate::Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut next_bytes = 0;
        for ones in data.iter().map(|&x| (x as u8).leading_ones()) {
            match (next_bytes == 0, ones) {
                (false, 1) => next_bytes -= 1,
                (true, 0 | 2 | 3 | 4) => next_bytes = ones.saturating_sub(1),
                _ => return false,
            }
        }
        next_bytes == 0
    }

    // https://leetcode.com/problems/utf-8-validation/discuss/644907/Rust-12-lines-short-solution-0ms
    pub fn valid_utf8_shifts(data: Vec<i32>) -> bool {
        let mut left = 0;
        for d in data.iter() {
            if left == 0 {
                if d >> 3 == 0b11110 {
                    left = 3
                } else if d >> 4 == 0b1110 {
                    left = 2
                } else if d >> 5 == 0b110 {
                    left = 1
                } else if d >> 7 == 0b0 {
                    left = 0
                } else {
                    return false;
                }
            } else {
                if d >> 6 != 0b10 {
                    return false;
                }
                left -= 1;
            }
        }
        left == 0
    }

    // https://leetcode.com/problems/utf-8-validation/discuss/2570191/Rust-or-Functional-State-Machine-or-With-Comments
    pub fn valid_utf8_functional(data: Vec<i32>) -> bool {
        data.into_iter()
            .map(|i| (i as u8).leading_ones())
            .scan((true, 0), |(ok, n), lo| match (*ok, *n, lo) {
                (false, _, _) => None,
                (_, 0, 0) => Some(true),
                (_, 0, 2 | 3 | 4) => {
                    *n = lo - 1;
                    Some(false)
                }
                (_, 1, 1) => {
                    *n = 0;
                    Some(true)
                }
                (_, _, 1) => {
                    *n -= 1;
                    Some(false)
                }
                _ => {
                    *ok = false;
                    Some(false)
                }
            })
            .last()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::valid_utf8(vec![250, 145, 145, 145, 145]));
    }
}

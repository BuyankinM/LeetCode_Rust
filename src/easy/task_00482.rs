// 482. License Key Formatting
// https://leetcode.com/problems/license-key-formatting/

use crate::Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let vs = s
            .split('-')
            .flat_map(|s| s.chars().map(|c| c.to_ascii_uppercase()))
            .collect::<Vec<_>>();

        let mut res = String::with_capacity(s.len());
        let mut part_size = match vs.len() % k as usize {
            0 => k,
            rem => rem as i32,
        };

        vs.into_iter().for_each(|c| {
            if part_size == 0 {
                res.push('-');
                part_size = k;
            }
            res.push(c);
            part_size -= 1;
        });
        res
    }

    // https://leetcode.com/problems/license-key-formatting/discuss/1662959/rust-iterators
    pub fn license_key_formatting_short(s: String, k: i32) -> String {
        s.to_ascii_uppercase()
            .replace('-', "")
            .as_bytes()
            .rchunks(k as usize)
            .rev()
            .map(String::from_utf8_lossy)
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "5F3Z-2E9W".to_string(),
            Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "2-5G-3J".to_string(),
            Solution::license_key_formatting("2-5g-3-J".to_string(), 2)
        );
    }
}

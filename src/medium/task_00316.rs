// 316. Remove Duplicate Letters
// https://leetcode.com/problems/remove-duplicate-letters/

use crate::Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counter: [usize; 26] = [0; 26];
        s.as_bytes()
            .iter()
            .for_each(|&b| counter[(b - b'a') as usize] += 1);

        let mut stack: Vec<u8> = Vec::with_capacity(26);
        let mut exists: [bool; 26] = [false; 26];

        for &b in s.as_bytes() {
            let idx = (b - b'a') as usize;
            counter[idx] -= 1;
            if exists[idx] {
                continue;
            }

            while let Some(&last) = stack.last() {
                let j = (last - b'a') as usize;
                if b < last && counter[j] > 0 {
                    exists[j] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(b);
            exists[idx] = true;
        }
        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "acdb".to_string(),
            Solution::remove_duplicate_letters("cbacdcbc".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "abc".to_string(),
            Solution::remove_duplicate_letters("bcabc".to_string())
        );
    }
}

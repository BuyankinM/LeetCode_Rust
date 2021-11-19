// 844. Backspace String Compare
// https://leetcode.com/problems/backspace-string-compare/

use crate::Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn get_str(s: String) -> Vec<char> {
            let mut stack = Vec::new();
            s.chars().for_each(|c| match c {
                '#' => {
                    stack.pop();
                }
                _ => stack.push(c),
            });
            stack
        }
        get_str(s) == get_str(t)
    }

    pub fn backspace_compare_pointers(s: String, t: String) -> bool {
        let next_char_pos = |a: &[u8], mut i: i32| -> i32 {
            let mut skip = 0;
            while i >= 0 && (skip > 0 || a[i as usize] == b'#') {
                skip += if a[i as usize] == b'#' { 1 } else { -1 };
                i -= 1;
            }
            i
        };

        let (mut i, mut j) = (s.len() as i32, t.len() as i32);
        let (sb, tb) = (s.as_bytes(), t.as_bytes());
        loop {
            i = next_char_pos(sb, i - 1);
            j = next_char_pos(tb, j - 1);

            if i < 0 || j < 0 || sb[i as usize] != tb[j as usize] {
                break;
            }
        }
        i == -1 && j == -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(Solution::backspace_compare_pointers(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::backspace_compare_pointers(
            "a##c".to_string(),
            "#a#c".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }
}

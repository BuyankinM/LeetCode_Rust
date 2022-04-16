// 383. Ransom Note
// https://leetcode.com/problems/ransom-note/

use crate::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut map = [0; 26];
        let mut flags = 0;
        let idx = |c: char| c as usize - 'a' as usize;

        ransom_note.chars().for_each(|c| {
            let i = idx(c);
            map[i] += 1;
            flags |= 1 << i; // set flag
        });

        for c in magazine.chars() {
            let i = idx(c);
            map[i] -= 1;
            if map[i] == 0 {
                flags ^= 1 << i; // drop flag
                if flags == 0 {
                    return true;
                }
            }
        }
        false
    }

    // https://leetcode.com/problems/ransom-note/discuss/610654/Rust-solution
    pub fn can_construct_opt(ransom_note: String, magazine: String) -> bool {
        let mut d: [usize; 256] = [0; 256];
        for c in magazine.chars().map(|c| (c as u8) as usize) {
            d[c] += 1;
        }
        for c in ransom_note.chars().map(|c| (c as u8) as usize) {
            if d[c] == 0 {
                return false;
            }
            d[c] -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }

    #[test]
    fn test_4() {
        assert!(Solution::can_construct("ab".to_string(), "aab".to_string()));
    }
}

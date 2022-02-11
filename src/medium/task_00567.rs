// 567. Permutation in String
// https://leetcode.com/problems/permutation-in-string/

use crate::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (mut counter_1, mut counter_2) = ([0; 26], [0; 26]);
        let l = s1.len();
        s1.as_bytes()
            .iter()
            .for_each(|&b| counter_1[(b - b'a') as usize] += 1);

        let sb_2 = s2.as_bytes();
        for (i, &b) in sb_2.iter().enumerate() {
            counter_2[(b - b'a') as usize] += 1;

            if i >= l {
                counter_2[(sb_2[i - l] - b'a') as usize] -= 1;
            }

            if i >= l - 1 && counter_1 == counter_2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::check_inclusion("a".to_string(), "ab".to_string()));
    }
}

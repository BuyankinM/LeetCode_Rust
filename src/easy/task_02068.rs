// 2068. Check Whether Two Strings are Almost Equivalent
// https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/

use crate::Solution;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let get_count = |s: &str| -> [i32; 26] {
            let mut count = [0; 26];
            s.as_bytes()
                .iter()
                .for_each(|&b| count[(b - b'a') as usize] += 1);
            count
        };
        let count_1 = get_count(&word1);
        let count_2 = get_count(&word2);

        count_1
            .iter()
            .zip(count_2.iter())
            .all(|(n1, n2)| (n1 - n2).abs() <= 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::check_almost_equivalent(
            "aaaa".to_string(),
            "bccb".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(Solution::check_almost_equivalent(
            "abcdeef".to_string(),
            "abaaacc".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::check_almost_equivalent(
            "cccddabba".to_string(),
            "babababab".to_string()
        ));
    }
}

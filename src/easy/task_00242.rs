// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/

use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        map.into_values().all(|v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_anagram("e".to_string(), "e".to_string()));
    }
}

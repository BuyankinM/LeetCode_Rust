// 1047. Remove All Adjacent Duplicates In String
// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/

use crate::Solution;

impl Solution {
    pub fn remove_duplicates_str(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            if let Some(cs) = stack.last() {
                if *cs == c {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c)
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "ca".to_string(),
            Solution::remove_duplicates_str("abbaca".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "ay".to_string(),
            Solution::remove_duplicates_str("azxxzy".to_string())
        );
    }
}
